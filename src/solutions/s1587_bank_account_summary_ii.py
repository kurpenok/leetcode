import pandas as pd


def account_summary(users: pd.DataFrame, transactions: pd.DataFrame) -> pd.DataFrame:
    return pd.merge(
        users,
        transactions.groupby("account")["amount"].sum().reset_index(name="balance"),
    ).query("balance > 10000")[["name", "balance"]]
