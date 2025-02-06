import pandas as pd


def sales_person(
    sales_person: pd.DataFrame, company: pd.DataFrame, orders: pd.DataFrame
) -> pd.DataFrame:
    try:
        red_id = company[(company.name == "RED")]["com_id"].iloc[0]
        orders = orders[(orders["com_id"] == red_id)]
        orders = orders[["sales_id"]]
        orders["flag"] = 1
        df = pd.merge(left=sales_person, right=orders, on="sales_id", how="left")
        return df[(df.flag.isna())][["name"]]
    except:
        return sales_person[["name"]]
