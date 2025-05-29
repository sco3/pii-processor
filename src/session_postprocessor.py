""" Module for processing session data after the session has been closed """

from pyarea22.log import logger

import cortex.background_tasks as bg
from cortex.chat_history import get_conversation_item_list
from cortex.config import env_vars as cfg
from cortex.config.env_vars import REDACT_LOGS
from cortex.models import session_state
from cortex.models.observer_models import ObserverRequest
from cortex.observer import ObserverClientSingleton
from cortex.utils.log_session import (
    generate_chat_history_string,
    get_redactions_from_llm,
    redact_session_log,
    save_chat_cloudwatch,
    save_session_log_cloudwatch,
)

observer_client = ObserverClientSingleton()


async def postprocess_session_state(session: session_state.SessionStateStorage) -> None:
    """
    Function to handle to postprocessing of the session.

    Args:
        session (session_state.SessionStateStorage): The session object

    """
    logger.debug("postprocess_session_state -->")

    # redact the session log for 2 destionations (cloduwatch and observer)
    messages = get_conversation_item_list(session, pop_unanswered=False)
    chat_log = generate_chat_history_string(messages, False)
    if REDACT_LOGS:
        chat_log_without_memories = generate_chat_history_string(messages, True)

        redactions = await get_redactions_from_llm(chat_log_without_memories)
        if redactions:
            # difference here is that redacted_log is in str format that got uploaded to Cloudwatch
            # and redacted_log_observer is a list of ConversationItem that is sent to observer for subsequent processing
            redacted_log = redact_session_log(chat_log, redactions)
            redacted_log_observer = redact_session_log(messages, redactions)
        else:
            redacted_log = chat_log
            redacted_log_observer = messages
    else:
        redacted_log = chat_log
        redacted_log_observer = messages
    bg.tasks.add(
        save_chat_cloudwatch(
            redacted_log,
            session.gpt_session_filename,
            session.session_id,
            session.user_id,
        ),
        f"save_cloudwatch_{session.session_id}",
        ignore_cancelled_error=True,
    )

    await save_session_log_cloudwatch(session.gpt_session_filename, session.session_id)

    if cfg.OBSERVER_ENABLED:
        if len(redacted_log_observer) > 0:
            await observer_client.invoke_observer(
                ObserverRequest(
                    profile_id=session.profile_id,
                    session_id=session.session_id,
                    observer_id=session.observer_id,
                    conversation=redacted_log_observer,
                )
            )

            await logger.adebug("<- postprocess_session_state: %s", session.session_id)
        else:
            await logger.adebug(
                "<- postprocess_session_state messages are empty : %s",
                session.session_id,
            )
