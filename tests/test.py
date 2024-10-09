import datetime
import rs_fsrs_python
from rs_fsrs_python import Rating, Card, FSRS, RecordLog

f = FSRS.default()
c = Card()

scheduled_cards = f.repeat(c, datetime.datetime.now(tz=datetime.timezone.utc))
for rating in [Rating.Again, Rating.Hard, Rating.Good, Rating.Easy]:
    updated_card = scheduled_cards.get(rating)
    print(updated_card.card)
    print(updated_card.review_log)
