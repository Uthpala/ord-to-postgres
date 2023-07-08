use super::*;
// use crate::sat_point::SatPoint;
use postgres::{Client as PGCLient, NoTls};
use std::error::Error;

// const URL: &str = "postgresql://postgres:postgres@localhost:5436/epp-db";
const URL: &str = "postgresql://postgres:n!FJcKwBR6buban@db.zbjfyhudmtqfxwajuuxg.supabase.co:5432/postgres"

/*
struct SatToSatpoint {
  pub sat: i64,
  pub satpoint: String,
}

async fn insert_sat_to_satpoint(
  sat_to_satpoint: &SatToSatpoint,
  pool: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
  let query = "INSERT INTO sat_to_satpoint (sat, satpoint) VALUES ($1, $2)";
  sqlx::query(query)
    .bind(&sat_to_satpoint.sat)
    .bind(&sat_to_satpoint.satpoint)
    .execute(pool)
    .await?;
  Ok(())
}

pub async fn insert_all_to_sat_to_satpoint(
  results: &Vec<(Sat, SatPoint)>,
) -> Result<(), Box<dyn Error>> {
  let url =
    "postgresql://postgres:n!FJcKwBR6buban@db.zbjfyhudmtqfxwajuuxg.supabase.co:5432/postgres";
  let pool = sqlx::postgres::PgPool::connect(url).await;
  match pool {
    Ok(value) => {
      for (sat, sat_point) in results.iter().rev().take(1000) {
        let sat_data = SatToSatpoint {
          sat: sat.n() as i64,
          satpoint: sat_point.to_string(),
        };
        let _try_to_create = insert_sat_to_satpoint(&sat_data, &value).await;
      }
    }
    Err(error) => {
      println!("error {:?}", error);
    }
  }
  Ok(())
}
 */

pub fn update_or_insert_sat_to_satpoint(sat: i64, satpoint: String) -> Result<(), Box<dyn Error>> {
  let mut client = PGCLient::connect(URL, NoTls)?;

  let update = client.execute(
    "UPDATE sat_to_satpoint SET satpoint = $2 WHERE sat = $1",
    &[&sat, &satpoint],
  )?;

  if update == 0 {
    client.execute(
      "INSERT INTO sat_to_satpoint (sat, satpoint) VALUES ($1, $2)",
      &[&sat, &satpoint],
    )?;
  }
  Ok(())
}

// pub fn update_or_insert_outpoint_to_sat_range(
//   outpoint: String,
//   sat_range_start: i64,
//   sat_range_end: i64,
// ) -> Result<(), Box<dyn Error>> {
//   let url =
//     "postgresql://postgres:n!FJcKwBR6buban@db.zbjfyhudmtqfxwajuuxg.supabase.co:5432/postgres";
//   let mut client = PGCLient::connect(url, NoTls)?;

//   let update = client.execute(
//     "UPDATE outpoint_to_sat_ranges SET sat_range_start = $1, sat_range_end = $2 WHERE outpoint = $3",
//     &[&sat_range_start, &sat_range_end, &outpoint],
//   )?;

//   if update == 0 {
//     client.execute(
//       "INSERT INTO outpoint_to_sat_ranges (outpoint, sat_range_start, sat_range_end) VALUES ($1, $2, $3)",
//       &[&outpoint, &sat_range_start, &sat_range_end],
//     )?;
//   }
//   Ok(())
// }

pub fn update_or_insert_inscription(
  inscription: String,
  address: String,
) -> Result<(), Box<dyn Error>> {
  let mut client = PGCLient::connect(URL, NoTls)?;

  let update = client.execute(
    "UPDATE inscriptions SET address = $2 WHERE inscription_id = $1",
    &[&inscription, &address],
  )?;

  if update == 0 {
    client.execute(
      "INSERT INTO inscriptions (inscription_id, address) VALUES ($1, $2)",
      &[&inscription, &address],
    )?;
  }

  Ok(())
}

// pub async fn insert_inscription_async(
//   inscription: String,
//   address: String,
//   pool: &sqlx::PgPool,
// ) -> Result<(), Box<dyn Error>> {
//   let query = "INSERT INTO inscriptions (inscription_id, address) VALUES ($1, $2)";
//   sqlx::query(query)
//     .bind(&inscription)
//     .bind(&address)
//     .execute(pool)
//     .await?;

//   Ok(())
// }

// pub fn update_or_insert_inscription_id_to_inscription_entry(
//   inscription_id: String,
//   fee: i64,
//   height: i64,
//   number: i64,
//   sat: i64,
//   timestamp: i64,
// ) -> Result<(), Box<dyn Error>> {
//   let url =
//     "postgresql://postgres:n!FJcKwBR6buban@db.zbjfyhudmtqfxwajuuxg.supabase.co:5432/postgres";
//   let mut client = PGCLient::connect(url, NoTls)?;

//   let update = client.execute(
//     "UPDATE inscription_id_to_inscription_entry SET fee = $1, height = $2, number = $3, sat = $4, timestamp = $5 WHERE inscription_id = $6",
//     &[ &fee, &height, &number, &sat, &timestamp, &inscription_id],
//   )?;

//   if update == 0 {
//     client.execute(
//       "INSERT INTO inscription_id_to_inscription_entry (fee, height, number, sat, timestamp, inscription_id ) VALUES ($1, $2, $3, $4, $5, $6)",
//       &[ &fee, &height, &number, &sat, &timestamp, &inscription_id],
//     )?;
//   }
//   Ok(())
// }
