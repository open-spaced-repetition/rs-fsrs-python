import datetime
import rs_fsrs_python
from rs_fsrs_python import Rating
c = rs_fsrs_python.Card()
f = rs_fsrs_python.FSRS()
t = datetime.datetime.now(tz=datetime.timezone.utc)
t = t.replace(tzinfo=datetime.timezone.utc)
scheduled_cards = f.schedule(c, t)
updated_card = scheduled_cards.select_card(Rating.Easy)
print(updated_card.log())
