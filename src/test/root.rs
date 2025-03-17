#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn load_env() {
        dotenv::from_filename(".env.example").ok();
    }
}
