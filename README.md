# thats-a-stack

**T**okio **H**tmx **A**xum **T**ailwind **S**qlx **A**skama stack.

A stack for making simple interactive web applications with just Rust and HTML templates that compiles into a single small-ish binary/docker image.
Includes a trivial counter component as an example for full-stack interaction.

## Development

### Structure
- `src/ui`: views
- `src/domain.rs`: domain types
- `src/app_state.rs`: database-domain interaction
- `templates`: askama templates
- `assets` and `src/assets.rs`: static assets need to be included in assets router
- `tailwind`: tailwind build files

### Frontend
- After modifying tailwind classes in templates run `npm run dev` to compile the styles

### Database
- Manage database migrations using `cargo sqlx migrate`
- After modifying queries run `cargo sqlx prepare` with a valid `DATABASE_URL` defined in `.env` to update `.sqlx` query cache

