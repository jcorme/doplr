use std::pin::Pin;

pub mod musicbrainz;

pub enum QueryExpr<'a> {
    Title(&'a str),
    Artist(&'a str),
    Artists(Vec<&'a str>),
    Album(&'a str),
    Year(u16),
    TrackNumber(u16),
    TotalTracks(u16),
    Length(u32),
    // Key => Value
    Custom(&'a str, &'a str),
    AND(Box<QueryExpr<'a>>, Box<QueryExpr<'a>>),
    OR(Box<QueryExpr<'a>>, Box<QueryExpr<'a>>),
}

impl<'a> QueryExpr<'a> {
    pub fn and(self, other: QueryExpr<'a>) -> QueryExpr<'a> {
        QueryExpr::AND(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: QueryExpr<'a>) -> QueryExpr<'a> {
        QueryExpr::OR(Box::new(self), Box::new(other))
    }
}

pub trait MetadataProvider {
    type Response: std::fmt::Debug;
    type Error: std::error::Error;

    // Until async traits are merged, must return a future here rather than
    // make the function async. No way to avoid the extra allocation(s).
    fn run_query<'a>(&self, q: QueryExpr<'a>)
        -> Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>>;

    fn build_query<'a>(q: QueryExpr<'a>) -> String;
}
