import pandas as pd


def big_countries(world: pd.DataFrame) -> pd.DataFrame:
    return world[(world["area"] >= 3_000_000) | (world["population"] >= 25_000_000)][
        ["name", "population", "area"]
    ]
