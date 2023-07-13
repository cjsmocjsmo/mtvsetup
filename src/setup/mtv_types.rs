use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: u32,
    pub name: String,
    pub year: String,
    pub posteraddr: String,
    pub size: String,
    pub path: String,
    pub idx: String,
    pub movid: String,
    pub catagory: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TVShow {
    pub id: u32,
    pub tvid: String,
    pub size: String,
    pub catagory: String,
    pub name: String,
    pub season: String,
    pub episode: String,
    pub path: String,
    pub idx: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieImage {
    pub id: u32,
    pub imgid: String,
    pub path: String,
    pub imgpath: String,
    pub size: String,
    pub name: String,
    pub thumbpath: String,
    pub idx: u32,
    pub httpthumbpath: String,
}