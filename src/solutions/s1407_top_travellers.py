import pandas as pd


def top_travellers(users: pd.DataFrame, rides: pd.DataFrame) -> pd.DataFrame:
    df = (
        rides.groupby(["user_id"])["distance"]
        .sum()
        .reset_index(name="travelled_distance")
    )

    df = pd.merge(users, df, how="left", left_on="id", right_on="user_id")

    df["travelled_distance"] = df["travelled_distance"].fillna(0)

    df.sort_values(
        by=["travelled_distance", "name"], ascending=[False, True], inplace=True
    )

    return df[["name", "travelled_distance"]]
