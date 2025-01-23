import pandas as pd


def duplicate_emails(person: pd.DataFrame) -> pd.DataFrame:
    person = person.groupby("email")["id"].count().reset_index()
    return person[person["id"] > 1][["email"]]
