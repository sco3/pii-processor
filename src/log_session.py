import asyncio
import base64
import boto3
import json
import lzma
import os
import re
import socket
import urllib.parse
from botocore.exceptions import ClientError
from common_data_models.conversation import ConversationItem
from cortex import gpt_clients
from cortex.agents.redaction.prompts import SYSTEM_PROMPT_STATUS_MESSAGE
from cortex.config.env_vars import (
    CLOUDWATCH_LOG_GROUP,
    ENCODE_CLOUDWATCH_LOGS,
    INCLUDE_USERID_IN_SESSIONS_LOG_FILENAME,
    LLM_REGION,
    POD_NAMESPACE,
    REDACT_LOGS,
    SESSIONS_LOG_URL,
)
from cortex.models import session_state
from datetime import datetime
from enum import StrEnum, auto
from llm_helper.conversation import ConversationRoles
from pathlib import PosixPath
from pyarea22.log import logger
from typing import Optional, Tuple


class StorageType(StrEnum):
    FILE = auto()
    S3 = auto()


USER_ROLE = "user"
S3_CLIENT = boto3.client("s3")
GPT_CLIENT = gpt_clients.gpt_client_factory()

if CLOUDWATCH_LOG_GROUP:
    CLOUDWATCH_LOGS_CLIENT = boto3.client("logs")

MAX_RETRIES = 3
RETRY_DELAY = 5
ERROR = "ERROR"


def save_file(content: str, filename: str) -> None:
    log_destination, storage_type = get_log_location()

    match storage_type:
        case StorageType.FILE:
            store_local_log(content, filename, log_destination)
        case StorageType.S3:
            s3_path = "%s/%s" % (log_destination, filename)
            upload_to_s3(content, s3_path)


def get_log_location() -> Tuple[str, str]:
    parsed_sessions_log_url = urllib.parse.urlparse(SESSIONS_LOG_URL)
    scheme = parsed_sessions_log_url.scheme

    match scheme:
        case StorageType.S3:
            bucket_name = parsed_sessions_log_url.netloc
            return (bucket_name, StorageType.S3)
        case "http" | "https":
            if "amazonaws" in parsed_sessions_log_url.netloc:
                bucket_name = parsed_sessions_log_url.netloc.split(".")[0]
                return (bucket_name, StorageType.S3)
            else:
                raise ValueError("Must be a S3 URL")
        case StorageType.FILE:
            log_dir = urllib.parse.unquote(parsed_sessions_log_url.path)
            os.makedirs(log_dir, exist_ok=True)
            return (log_dir, StorageType.FILE)

    raise ValueError("Unsupported URL scheme '%s' for SESSIONS_LOG_URL" % scheme)


def upload_to_s3(content: str, s3_path: str) -> None:
    if POD_NAMESPACE != "undefined host":
        hostname = POD_NAMESPACE
    else:
        hostname = socket.gethostname()
    bucket_name, s3_key = s3_path.split("/", 1)
    response = S3_CLIENT.put_object(
        Body=content, Bucket=bucket_name, Key=s3_key, Metadata={"host": hostname}
    )
    logger.info("Uploaded file to S3: %s/%s", bucket_name, s3_key)
    return response


def store_local_log(content: str, filename: str, dir_path: str) -> None:
    full_path = PosixPath(dir_path).expanduser()
    if not full_path.exists():
        logger.info("Creating directory: %s", full_path)
        full_path.mkdir(parents=True, exist_ok=True)
    full_filename = full_path / filename
    full_filename.write_text(content, encoding="utf-8")


def generate_chat_history_string(messages: list, strip_memories: bool) -> str:
    chat_log = ""
    for message in messages:
        if isinstance(message, dict):
            role = message["role"]
            content = message["content"]
        else:
            role = message.role
            content = message.content

        # skip system instructions
        if role == ConversationRoles.SYSTEM.value:
            continue

        if role == ConversationRoles.USER.value:
            if strip_memories and "<conversation_memories" in content:
                pre_content = content.split("<conversation_memories")[0].strip()
                if "</conversation_memories>" in content:
                    post_content = content.split("</conversation_memories>")[1].strip()
                    content = (
                        pre_content + " " + post_content
                        if pre_content and post_content
                        else pre_content or post_content
                    )
                else:
                    content = pre_content

            if "<user_input>" in content:
                user_input = (
                    content.split("<user_input>")[1].split("</user_input>")[0].strip()
                )
                chat_log += f"{ConversationRoles.USER.value}: {user_input}\n"
            else:
                chat_log += f"{ConversationRoles.USER.value}: {content}\n"
        elif role == ConversationRoles.ASSISTANT.value:
            if "<assistant_response>" in content:
                assistant_response = (
                    content.split("<assistant_response>")[1]
                    .split("</assistant_response>")[0]
                    .strip()
                )
                chat_log += (
                    f"{ConversationRoles.ASSISTANT.value}: {assistant_response}\n"
                )
            else:
                cleaned_content = content.strip('"')
                chat_log += f"{ConversationRoles.ASSISTANT.value}: {cleaned_content}\n"
        else:
            chat_log += f"{ConversationRoles.ASSISTANT.value}: {content}\n"

    return chat_log


def redact_session_log(
    content: list | dict | str, redactions: dict
) -> list | dict | str:
    if isinstance(content, str):
        redacted_content = content
        for original, replacement in redactions.items():
            pattern = r"(?<!\w)" + re.escape(original) + r"(?!\w)"
            redacted_content = re.sub(pattern, replacement, redacted_content)
        return redacted_content

    elif isinstance(content, list):
        for i, item in enumerate(content):
            if isinstance(item, ConversationItem):
                item.content = redact_session_log(item.content, redactions)
            else:
                content[i] = redact_session_log(item, redactions)
        return content

    elif isinstance(content, dict):
        redacted_dict = {}
        for key, value in content.items():
            redacted_dict[key] = redact_session_log(value, redactions)
        return redacted_dict

    else:
        return content


async def get_redactions_from_llm(text: str) -> dict:
    messages = [
        {"role": "system", "content": SYSTEM_PROMPT_STATUS_MESSAGE},
        {"role": "user", "content": text},
    ]

    retries = 0
    while retries < MAX_RETRIES:
        try:
            response = await GPT_CLIENT.invoke(
                messages,
                tools=[],
                aws_region_name=LLM_REGION,
                temperature=0,
            )
            parsed_response = json.loads(response.content)
            redactions = parsed_response.get("redactions", {})
            # Sometimes LLM uses redactions not specified in the
            # redaction system prompt. Thus, we need to filter
            # out unwanted redactions.
            values_to_filter = ["[DEVICE]", "[DEVICE_ID]", "[DEVICE_NAME]"]
            filtered_redactions = {
                key: value
                for key, value in redactions.items()
                if value not in values_to_filter
            }
            return filtered_redactions
        except json.JSONDecodeError:
            logger.warning(
                f"Failed to parse LLM response as JSON for redaction (attempt {retries + 1})"
            )
            messages = [
                {"role": "system", "content": SYSTEM_PROMPT_STATUS_MESSAGE},
                {"role": "user", "content": text},
                {"role": "system", "content": response.content},
                {
                    "role": "user",
                    "content": "You haven't responded in JSON format as instructed. Please respond in JSON",
                },
            ]
        except Exception as e:
            logger.error(f"LLM call failed (attempt {retries + 1}): {e}")

        retries += 1
        if retries < MAX_RETRIES:
            await asyncio.sleep(RETRY_DELAY)

    logger.error("Max retries reached. Returning empty redactions.")
    return {ERROR: ERROR}


async def redact_and_save(content: str | dict, filename: str, messages: list) -> None:
    if REDACT_LOGS:
        chat_history = generate_chat_history_string(messages, True)
        redactions = await get_redactions_from_llm(chat_history)
        if ERROR in redactions:
            logger.warning("PII redaction failed")
        if redactions:
            content = redact_session_log(content, redactions)

    content_to_save = (
        json.dumps(content, indent=4) if isinstance(content, (dict, list)) else content
    )

    save_file(content_to_save, filename)


async def save_gpt_session(
    request_state: session_state.RequestState, error=None
) -> None:
    logger.debug("--> save_gpt_session()")
    user_label = get_user_label(request_state) or "unknown_user"

    if (
        request_state.gpt_session_log
        and request_state.session.gpt_session_filename
        and not error
    ):
        logger.debug(
            "--> save_gpt_session(), current session log filename:%s",
            request_state.session.gpt_session_filename,
        )
        await redact_and_save(
            request_state.gpt_session_log,
            request_state.session.gpt_session_filename,
            request_state.session.messages,
        )
    elif error:
        request_state.session.gpt_session_filename = f"gpt_session_{request_state.session.session_id}_{user_label}_conversation_turn_{request_state.session.conversation_turn}"
        logger.debug(
            "Creating an error log file %s", request_state.session.gpt_session_filename
        )
        await redact_and_save(
            error,
            request_state.session.gpt_session_filename,
            request_state.session.messages,
        )


def save_telus_api_log_session(request_state: session_state.RequestState) -> None:
    """Function to save the Telus API session log information."""

    if (
        request_state.telus_api_session_log
        and request_state.session.telus_api_log_session_filename
    ):
        logger.debug(
            "--> save_telus_api_log_session(), current session log filename: %s",
            request_state.session.telus_api_log_session_filename,
        )

        content_conversation = json.dumps(request_state.telus_api_session_log, indent=4)
        # logger.debug(f"save_gpt_session log: {content_conversation}")
        save_file(
            content_conversation,
            request_state.session.telus_api_log_session_filename,
        )


def get_user_label(request_state: session_state.RequestState) -> str:
    """Helper function to determine user label from the request state"""
    return (
        request_state.session.user_id
        if INCLUDE_USERID_IN_SESSIONS_LOG_FILENAME and request_state.session.user_id
        else request_state.session.profile_id
    )


def build_session_log_filenames(
    request_state: session_state.RequestState,
) -> str | None:
    """Function to build the session log filenames."""

    logger.debug("Conversation turn: %s", request_state.session.conversation_turn)
    if request_state.gpt_session_log:
        build_gpt_session_log_filename(request_state)
    if request_state.telus_api_session_log:
        build_telus_api_log_session_filename(request_state)


def build_gpt_session_log_filename(request_state: session_state.RequestState) -> None:
    """Function to build the GPT session log filename."""
    request_state.session.gpt_session_filename = f"gpt_session_{request_state.session.session_id}_{get_user_label(request_state)}_conversation_turn_{request_state.session.conversation_turn}"
    logger.debug(
        "Creating new gpt-session conversation turn file %s",
        request_state.session.gpt_session_filename,
    )


def build_telus_api_log_session_filename(
    request_state: session_state.RequestState,
) -> None:
    """Function to build the Telus API session log filename."""
    # state.session.conversation_turn += 1
    request_state.session.telus_api_log_session_filename = f"telus_api_log_session_{request_state.session.session_id}_{get_user_label(request_state)}_conversation_turn_{request_state.session.conversation_turn}"
    logger.debug(
        "Creating new telus-api conversation turn file %s",
        request_state.session.telus_api_log_session_filename,
    )


def get_session_log_filename(request_state: session_state.RequestState) -> str:
    """get the session filename."""
    logger.debug(
        "Include session log filename %s",
        request_state.session.gpt_session_filename,
    )
    return request_state.session.gpt_session_filename


async def save_chat_cloudwatch(
    chat_details: str, file_name: str, session_id: str, user_id: str
) -> None:

    chat_details = add_cloudwatch_chat_history_markup(
        chat_details, file_name, session_id, user_id
    )

    if not CLOUDWATCH_LOG_GROUP:
        logger.info(chat_details)
        return

    log_stream = f"chat_history_{session_id}_{user_id}"
    log_group = f"{CLOUDWATCH_LOG_GROUP}_chat_history"
    ensure_log_group_exists(log_group)
    log_event = {
        "timestamp": int(datetime.utcnow().timestamp() * 1000),
        "message": chat_details,
    }

    sequence_token = upload_to_cloudwatch_log_stream(log_stream, log_event)

    # If a token was returned, it means we need to retry with the correct token
    if sequence_token:
        kwargs = {
            "logGroupName": log_group,
            "logStreamName": log_stream,
            "logEvents": [log_event],
            "sequenceToken": sequence_token,
        }

        try:
            logger.info(f"Retrying to upload to Cloudwatch: {log_stream}")
            CLOUDWATCH_LOGS_CLIENT.put_log_events(**kwargs)
            logger.info(
                f"Successfully uploaded to CloudWatch Logs stream (retry): {log_stream}"
            )
        except Exception as e:
            logger.error(f"Failed to upload to CloudWatch even after retry: {str(e)}")


def upload_to_cloudwatch_log_stream(log_stream: str, log_event: dict) -> Optional[str]:
    """
    Upload a log event to a CloudWatch log stream.

    Args:
        log_stream: The name of the CloudWatch log stream
        log_event: The log event to upload

    Returns:
        bool: True if successful, False if failed
    """
    log_group = f"{CLOUDWATCH_LOG_GROUP}_chat_history"
    try:
        CLOUDWATCH_LOGS_CLIENT.create_log_stream(
            logGroupName=log_group, logStreamName=log_stream
        )
        sequence_token = None
    except CLOUDWATCH_LOGS_CLIENT.exceptions.ResourceAlreadyExistsException:
        response = CLOUDWATCH_LOGS_CLIENT.describe_log_streams(
            logGroupName=log_group,
            logStreamNamePrefix=log_stream,
            limit=1,
        )
        sequence_token = response["logStreams"][0].get("uploadSequenceToken")

    kwargs = {
        "logGroupName": log_group,
        "logStreamName": log_stream,
        "logEvents": [log_event],
    }

    if sequence_token:
        kwargs["sequenceToken"] = sequence_token

    try:
        CLOUDWATCH_LOGS_CLIENT.put_log_events(**kwargs)
        logger.info(f"Successfully uploaded to CloudWatch Logs stream: {log_stream}")
    except CLOUDWATCH_LOGS_CLIENT.exceptions.InvalidSequenceTokenException as e:
        logger.error("Invalid sequence token when uploading to CloudWatch: %s", str(e))
        return e.response["expectedSequenceToken"]


def create_entity_chunk(items: list[dict], max_size: int) -> list[list[dict]]:
    """Create entity chunks that fit within CloudWatch size limits.
    Returns a list of chunks, where each chunk is a list of items."""
    if not items:
        return []

    if len(items) == 1:
        return chunk_large_item(items[0], max_size)

    mid = len(items) // 2
    first_half = items[:mid]
    second_half = items[mid:]

    total_size = sum(len(json.dumps(item).encode("utf-8")) for item in items)
    if total_size <= max_size:
        return [items]
    else:
        return create_entity_chunk(first_half, max_size) + create_entity_chunk(
            second_half, max_size
        )


def chunk_large_item(item: dict, max_size: int) -> list[list[dict]]:
    """Split a large item into separate entity chunks.
    Each inner list represents a single CloudWatch entity.
    """
    if "content" in item and isinstance(item["content"], str):
        content = item["content"]
        chunks = [content[i : i + max_size] for i in range(0, len(content), max_size)]
        return [[item.copy() | {"content": chunk}] for chunk in chunks]
    else:
        json_str = json.dumps(item)
        chunks = [json_str[i : i + max_size] for i in range(0, len(json_str), max_size)]
        return [[{"raw_content": chunk}] for chunk in chunks]


def chunk_content(content: list[dict], max_size: int = 245 * 1024) -> list[list[dict]]:
    """Split content into CloudWatch entity chunks.
    Returns a list of entity chunks, where each chunk is a list of dicts
    that will become a single CloudWatch log event.
    """
    entity_chunks = []
    current_items = []
    current_size = 0

    for item in content:
        # Calculate size of JSON string in bytes.
        item_size = len(json.dumps(item).encode("utf-8"))

        # If single item is too large, split it
        if item_size > max_size:
            # Large items become their own entity chunks
            entity_chunks.extend(chunk_large_item(item, max_size))
            continue

        if current_size + item_size > max_size:
            # Create new entity chunk from accumulated items
            if current_items:
                entity_chunks.append(create_entity_chunk(current_items, max_size))
            current_items = [item]
            current_size = item_size
        else:
            current_items.append(item)
            current_size += item_size

    if current_items:
        entity_chunks.extend(create_entity_chunk(current_items, max_size))

    return entity_chunks


def upload_chunk_to_cloudwatch(
    log_group: str, log_stream: str, log_event: dict, sequence_token: str = None
) -> str:
    """Upload a single chunk to CloudWatch and return next sequence token."""
    kwargs = {
        "logGroupName": log_group,
        "logStreamName": log_stream,
        "logEvents": [log_event],
    }
    if sequence_token:
        kwargs["sequenceToken"] = sequence_token

    try:
        response = CLOUDWATCH_LOGS_CLIENT.put_log_events(**kwargs)
        return response["nextSequenceToken"]
    except CLOUDWATCH_LOGS_CLIENT.exceptions.InvalidSequenceTokenException as e:
        logger.error("Invalid sequence token when uploading to CloudWatch: %s", str(e))
        return e.response["expectedSequenceToken"]


async def get_session_data_from_s3(filename: str) -> dict:
    """Retrieve and parse session data from S3 with retries for file-not-found errors."""
    bucket_name, _ = get_log_location()
    retries = 5
    delay = 10
    last_exception = None

    for attempt in range(0, retries + 1):
        try:
            response = S3_CLIENT.get_object(Bucket=bucket_name, Key=filename)
            data = response["Body"].read().decode("utf-8")
            return json.loads(data)
        except ClientError as e:
            error_code = e.response.get("Error", {}).get("Code")
            if error_code == "NoSuchKey":
                last_exception = e
                logger.warning(
                    "Attempt %d: File '%s' not found in bucket '%s'. Retrying in %d seconds...",
                    attempt,
                    filename,
                    bucket_name,
                    delay,
                )
                await asyncio.sleep(delay)
            else:
                raise

    raise Exception(
        f"Failed to retrieve session data from S3 after {retries} attempts: file '{filename}' not found"
    ) from last_exception


def ensure_log_group_exists(log_group_name: str) -> None:
    """Ensure the CloudWatch log group exists, creating it if necessary."""
    try:
        CLOUDWATCH_LOGS_CLIENT.create_log_group(logGroupName=log_group_name)
        print(f"Created log group: {log_group_name}")
    except CLOUDWATCH_LOGS_CLIENT.exceptions.ResourceAlreadyExistsException:
        pass
    except Exception as e:
        print(f"Error creating log group: {str(e)}")
        raise


def upload_chunks_to_cloudwatch(
    content_chunks: list[list[dict]],
    session_id: str,
    filename: str,
    log_stream: str,
    log_group: str,
) -> None:
    """Upload multiple chunks to CloudWatch with metadata."""
    total_chunks = len(content_chunks)
    sequence_token = None
    uploaded_chunks = 0

    for chunk_number, chunk in enumerate(content_chunks, 1):
        metadata = {
            "session_id": session_id,
            "chunk_number": chunk_number,
            "total_chunks": total_chunks,
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "filename": filename,
        }

        chunk_data = {"metadata": metadata, "content": chunk}
        log_event = {
            "timestamp": int(datetime.utcnow().timestamp() * 1000),
            "message": json.dumps(chunk_data),
        }

        try:
            sequence_token = upload_chunk_to_cloudwatch(
                log_group, log_stream, log_event, sequence_token
            )
            uploaded_chunks += 1
        except (
            CLOUDWATCH_LOGS_CLIENT.exceptions.InvalidSequenceTokenException,
            CLOUDWATCH_LOGS_CLIENT.exceptions.ResourceNotFoundException,
            CLOUDWATCH_LOGS_CLIENT.exceptions.InvalidParameterException,
            CLOUDWATCH_LOGS_CLIENT.exceptions.ServiceUnavailableException,
        ) as e:
            logger.error(
                "Failed to upload chunk %d/%d: %s", chunk_number, total_chunks, str(e)
            )
            continue

    if uploaded_chunks == total_chunks:
        logger.info("Successfully uploaded %d chunks to CloudWatch Logs", total_chunks)
    else:
        logger.error(
            "only uploaded %d out of %d chunks to cloudwatch",
            uploaded_chunks,
            total_chunks,
        )


async def save_session_log_cloudwatch(filename: str, session_id: str) -> None:
    """Main function to save session logs to CloudWatch."""
    if not CLOUDWATCH_LOG_GROUP:
        return

    try:
        session_data = await get_session_data_from_s3(filename)
    except Exception as e:
        logger.warning(
            "Unable to retrieve session data from S3; skipping CloudWatch upload: %s",
            str(e),
        )
        return

    log_stream = f"{session_id}_{datetime.utcnow().strftime('%Y%m%d_%H%M%S')}"

    ensure_log_group_exists(CLOUDWATCH_LOG_GROUP)

    try:
        CLOUDWATCH_LOGS_CLIENT.create_log_stream(
            logGroupName=CLOUDWATCH_LOG_GROUP, logStreamName=log_stream
        )
    except CLOUDWATCH_LOGS_CLIENT.exceptions.ResourceAlreadyExistsException:
        pass

    if ENCODE_CLOUDWATCH_LOGS:
        session_data_json = json.dumps(session_data)
        compressed_data = lzma.compress(session_data_json.encode())
        encoded_data = base64.b64encode(compressed_data).decode()
        content_chunks = chunk_content([{"encoded_data": encoded_data}])
    else:
        content_chunks = chunk_content([session_data])

    upload_chunks_to_cloudwatch(
        content_chunks=content_chunks,
        session_id=session_id,
        filename=filename,
        log_stream=log_stream,
        log_group=CLOUDWATCH_LOG_GROUP,
    )


def add_cloudwatch_chat_history_markup(
    chat_details: str, file_name: str, session_id: str, user_id: str
) -> str:
    """Add markup to chat history before uploading to Cloudwatch."""

    chat_details += f"\nLatest turn: {SESSIONS_LOG_URL}/{file_name}\n"
    chat_details += f"Session ID: {session_id}\n"
    chat_details += f"Profile ID: {user_id}"

    return chat_details
