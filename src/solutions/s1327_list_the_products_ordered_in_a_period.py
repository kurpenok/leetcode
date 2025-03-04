import pandas as pd


def list_products(products: pd.DataFrame, orders: pd.DataFrame) -> pd.DataFrame:
    february_2020_orders = orders[orders["order_date"].dt.year == 2020]
    february_2020_orders = february_2020_orders[orders["order_date"].dt.month == 2]

    merged = pd.merge(february_2020_orders, products, on="product_id")
    result = merged.groupby("product_name")["unit"].sum().reset_index()
    result = result[result["unit"] >= 100]

    return result[["product_name", "unit"]]
