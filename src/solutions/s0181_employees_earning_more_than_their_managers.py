import pandas as pd


def find_employees(employee: pd.DataFrame) -> pd.DataFrame:
    df = pd.merge(
        left=employee, right=employee, left_on="id", right_on="managerId", how="right"
    )
    return df[(df.salary_y > df.salary_x)][["name_y"]].rename(
        columns={"name_y": "Employee"}
    )
