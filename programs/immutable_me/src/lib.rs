use anchor_lang::prelude::*;

// ID de nuestro programa en Devnet
declare_id!("E4MYbLQLa1u3KFucSFjjR3c5jzNxtsbpmtkRa5EXXnd1");

#[program]
pub mod solana_portfolio {
    use super::*;

    // C: CREATE
    pub fn create_profile(
        ctx: Context<CreateProfile>,
        nickname: String,
        bio: String,
        website: String,
    ) -> Result<()> {
        // Le puse estos limites de caracteres para no reservar espacio de mas en la PDA
        // y asi no gastar tanto SOL en la renta de la cuenta.
        require!(nickname.len() <= 30, ProfileError::TextTooLong);
        require!(bio.len() <= 100, ProfileError::TextTooLong);
        require!(website.len() <= 100, ProfileError::TextTooLong);

        // Usamos mutable para poder escribir datos nuevos en la cuenta
        let profile = &mut ctx.accounts.profile;

        // Asignamos los datos que nos manda el usuario a la cuenta de la blockchain
        profile.owner = ctx.accounts.user.key();
        profile.nickname = nickname;
        profile.bio = bio;
        profile.website = website;

        // Un mensajito en los logs para confirmar que todo salio bien
        msg!("¡Perfil creado exitosamente para: {}!", profile.nickname);
        Ok(())
    }

    // R: READ
    // Ahora le pasamos el nickname aqui para que la pestaña de Test nos pida el texto
    pub fn view_profile(_ctx: Context<ViewProfile>, nickname: String) -> Result<()> {
        // En esta funcion solo leemos, por eso no necesitamos que sea mutable
        let profile = &_ctx.accounts.profile;

        // Imprimimos todo para verlo en la consola de Solana
        msg!("--- Buscando Perfil de {} ---", nickname);
        msg!("Bio: {}", profile.bio);
        msg!("Web: {}", profile.website);
        msg!("Dueño: {}", profile.owner);

        Ok(())
    }

    // U: UPDATE
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        nickname: String,
        new_bio: String,
        new_website: String,
    ) -> Result<()> {
        // Validamos de nuevo para que no se pasen del tamaño de la cuenta
        require!(new_bio.len() <= 100, ProfileError::TextTooLong);
        require!(new_website.len() <= 100, ProfileError::TextTooLong);

        let profile = &mut ctx.accounts.profile;

        // Aqui sobreescribimos los campos con la nueva informacion
        profile.bio = new_bio;
        profile.website = new_website;

        msg!("¡Perfil de {} actualizado correctamente!", nickname);
        Ok(())
    }

    // D: DELETE
    pub fn delete_profile(_ctx: Context<DeleteProfile>, nickname: String) -> Result<()> {
        // Al cerrar la cuenta, Anchor nos devuelve los SOL que habiamos dejado en deposito
        msg!(
            "Perfil {} borrado. Recuperaste tus SOL de la renta.",
            nickname
        );
        Ok(())
    }
}

// manejo de errorres
#[error_code]
pub enum ProfileError {
    #[msg("El texto es muy largo. ¡Cuidado con el espacio, que en la blockchain cuesta!")]
    TextTooLong,
}

// estructura de datos
#[account]
// InitSpace calcula automaticamente el tamaño basandose en los max_len
#[derive(InitSpace)]
pub struct UserProfile {
    pub owner: Pubkey, // 32 bytes de la wallet del creador
    #[max_len(30)]
    pub nickname: String, // longitud maxima definida para el nombre
    #[max_len(100)]
    pub bio: String, // espacio reservado para la descripcion
    #[max_len(100)]
    pub website: String, // espacio reservado para el link
}

// contextos seguridad y reglas de las cuentas

#[derive(Accounts)]
// El instruction permite usar argumentos de la funcion en los seeds
#[instruction(nickname: String)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // La wallet que firma y paga la transaccion

    #[account(
        init, // Crea la cuenta por primera vez en la red
        payer = user, 
        space = 8 + UserProfile::INIT_SPACE, // 8 bytes de discriminador + datos
        // PDA unica por usuario y nickname usando semillas
        seeds = [b"profile", user.key().as_ref(), nickname.as_bytes()], 
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>, // Programa base de Solana para crear cuentas
}

#[derive(Accounts)]
#[instruction(nickname: String)] // Le decimos que vamos a recibir un texto
pub struct ViewProfile<'info> {
    // Necesitamos pasarle la wallet del dueño para que Anchor sepa que perfil buscar
    pub owner: AccountInfo<'info>,

    #[account(
        // Solo verificamos que la cuenta exista con estas semillas calculadas
        seeds = [b"profile", owner.key().as_ref(), nickname.as_bytes()], 
        bump
    )]
    pub profile: Account<'info, UserProfile>,
}

#[derive(Accounts)]
#[instruction(nickname: String)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // La wallet que intenta actualizar

    #[account(
        mut, // Marcamos como mutable para permitir cambios de datos
        seeds = [b"profile", user.key().as_ref(), nickname.as_bytes()], 
        bump,
        // Restriccion: el campo owner de la cuenta debe ser igual a la wallet user
        constraint = profile.owner == user.key()
    )]
    pub profile: Account<'info, UserProfile>,
}

#[derive(Accounts)]
#[instruction(nickname: String)]
pub struct DeleteProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // Wallet que solicita el cierre de la cuenta

    #[account(
        mut,
        close = user, // Esta linea borra la cuenta y manda el SOL de la renta al user
        seeds = [b"profile", user.key().as_ref(), nickname.as_bytes()], 
        bump
    )]
    pub profile: Account<'info, UserProfile>,
}
