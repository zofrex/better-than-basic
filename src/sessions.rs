extern crate lru_cache;
extern crate rand;

use self::rand::OsRng;
use self::rand::Rng;
use std::io::Error;
use self::lru_cache::LruCache;

use iron::typemap::Key;

impl Key for Sessions {
    type Value = Sessions;
}

pub struct Sessions {
    rng: OsRng,
    sessions: LruCache<String, bool>,
}

impl Sessions {
    pub fn new() -> Result<Sessions, Error> {
        Ok(Sessions {
            rng: try!(OsRng::new()),
            sessions: LruCache::new(100),
        })
    }

    pub fn create_session(&mut self) -> String {
        let session_id = self.rng.gen_ascii_chars().take(50).collect::<String>();
        self.sessions.insert(session_id.clone(), true);
        session_id
    }

    pub fn check_session(&mut self, session_id: &str) -> bool {
        self.sessions.contains_key(session_id)
    }
}
