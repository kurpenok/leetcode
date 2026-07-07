pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    seats.sort();

    let mut students = students;
    students.sort();

    seats
        .iter()
        .zip(students)
        .map(|(seat, student)| (seat - student).abs())
        .sum()
}
