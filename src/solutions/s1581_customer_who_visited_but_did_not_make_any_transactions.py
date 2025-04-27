import pandas as pd


def find_customers(visits: pd.DataFrame, transactions: pd.DataFrame) -> pd.DataFrame:
    df = visits.merge(transactions, how="left")
    return (
        df[df["transaction_id"].isna()]
        .groupby(["customer_id"], as_index=False)
        .agg(count_no_trans=("visit_id", "nunique"))
    )
