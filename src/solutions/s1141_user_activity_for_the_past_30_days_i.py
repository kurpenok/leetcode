import pandas as pd


def user_activity(activity: pd.DataFrame) -> pd.DataFrame:
    return (
        activity[activity.activity_date.between("2019-06-28", "2019-07-27")]
        .rename(columns={"activity_date": "day", "user_id": "active_users"})
        .groupby("day")["active_users"]
        .nunique()
        .reset_index()
    )
