#[derive(Debug)]
pub struct Movie {
    pub name: String,
    pub year: String,
    pub poster_addr: String,
    pub size: String,
    pub exists: bool,
    pub path: String,
    pub index: String,
    pub movid: String,
}
// let tvshows_obj = object! {
    //     size: file_size,
    //     catagory: catagory,
    //     name: fname,
    //     season: season,
    //     episode: episode,
    //     path: tv
    // };
#[derive(Debug)]
pub struct TVShow {
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
    pub path: String,
    pub imgpath: String,
    pub size: String,
    pub name: String,
    pub thumbpath: String,
    pub idx: u32,
}