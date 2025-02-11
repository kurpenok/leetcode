import pandas as pd


def swap_salary(salary: pd.DataFrame) -> pd.DataFrame:
    return salary.replace({"f": "m", "m": "f"})
