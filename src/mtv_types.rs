#[derive(Debug)]
pub struct Movie {
    pub id: u32,
    pub name: String,
    pub year: String,
    pub posteraddr: String,
    pub size: String,
    pub exists: String,
    pub path: String,
    pub idx: String,
    pub movid: String,
}

#[derive(Debug)]
pub struct TVShow {
    pub tvid: String,
    pub size: String,
    pub catagory: String,
    pub name: String,
    pub season: String,
    pub episode: String,
    pub path: String,
    pub idx: String,

}

#[derive(Debug)]
pub struct MovieImage {
    pub imgid: String,
    pub path: String,
    pub imgpath: String,
    pub size: String,
    pub name: String,
    pub thumbpath: String,
    pub idx: u32,
}