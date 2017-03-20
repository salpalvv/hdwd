use chrono;

#[derive(Queryable, Debug)]
pub struct Products {
    pub id:             i32,
    pub name:           String,
    pub artifact:       String,
    pub environment:    String,
    pub version:        String,
    pub deployed:       chrono::NaiveDateTime,
}
