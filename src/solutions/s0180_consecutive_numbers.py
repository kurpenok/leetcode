import pandas as pd


def consecutive_numbers(logs: pd.DataFrame) -> pd.DataFrame:
    logs.sort_values(["id"], inplace=True)

    logs = logs[
        (logs.num == logs.num.shift(1))
        & (logs.num == logs.num.shift(2))
        & (logs.id == logs.id.shift(1) + 1)
        & (logs.id == logs.id.shift(2) + 2)
    ].drop_duplicates("num")

    return logs.iloc[:, [1]].rename(columns={"num": "ConsecutiveNums"})
