import logging
import os

from telegram import Update, ForceReply
from telegram.ext import Updater, CommandHandler, MessageHandler, Filters, CallbackContext

from .dice import answer

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def roll(update: Update, _: CallbackContext) -> None:
    update.message.reply_text(answer(update.message.text), quote=True)

def main() -> None:
    try:
        token = os.environ["TELEGRAM_BOT_TOKEN"]
    except KeyError as e:
        raise ValueError("missing TELEGRAM_BOT_TOKEN in environment") from e
    updater = Updater(token)
    dispatcher = updater.dispatcher

    dispatcher.add_handler(CommandHandler("roll", roll))
    dispatcher.add_handler(MessageHandler(Filters.update.message & ~Filters.command, roll))
    
    updater.start_polling()
    updater.idle()
