import pandas as pd


def average_selling_price(
    prices: pd.DataFrame, units_sold: pd.DataFrame
) -> pd.DataFrame:
    df = pd.merge(prices, units_sold, on="product_id", how="left")
    df = df[
        df.purchase_date.isna()
        | ((df.purchase_date >= df.start_date) & (df.purchase_date <= df.end_date))
    ]
    return (
        df.groupby("product_id")
        .apply(
            lambda x: round((x["price"] * x["units"]).sum() / x["units"].sum(), 2)
            if x["units"].sum() != 0
            else 0
        )
        .reset_index(name="average_price")
    )
