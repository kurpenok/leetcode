import pandas as pd


def reformat_table(department: pd.DataFrame) -> pd.DataFrame:
    prefixes = [
        "Jan",
        "Feb",
        "Mar",
        "Apr",
        "May",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec",
    ]

    bymonth = department.pivot(index="id", columns="month", values="revenue")
    bymonth = bymonth.reindex(columns=prefixes)
    bymonth.rename(columns=lambda prefix: prefix + "_Revenue", inplace=True)
    bymonth.reset_index(inplace=True)

    return bymonth
