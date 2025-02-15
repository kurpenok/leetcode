import pandas as pd


def sales_analysis(product: pd.DataFrame, sales: pd.DataFrame) -> pd.DataFrame:
    sales = sales.groupby("product_id")["sale_date"].agg(["min", "max"]).reset_index()
    sales = sales[(sales["min"] >= "2019-01-01") & (sales["max"] <= "2019-03-31")]
    return pd.merge(sales, product, on="product_id", how="inner")[
        ["product_id", "product_name"]
    ]
