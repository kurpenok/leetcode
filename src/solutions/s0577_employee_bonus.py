import pandas as pd


def employee_bonus(df1: pd.DataFrame, df2: pd.DataFrame) -> pd.DataFrame:
    df = pd.merge(left=df1, right=df2, on="empId", how="left")
    return df[(df.bonus < 1000) | (df.bonus.isna())][["name", "bonus"]]
