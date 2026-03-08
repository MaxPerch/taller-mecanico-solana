use anchor_lang::prelude::*;

declare_id!("h5VtPHbiU733cfx1frG9th2jQDX8BPBUfLReb8K9UFo");

#[program]
pub mod taller_mecanico {
    use super::*;

    pub fn crear_reparacion(
        ctx: Context<CrearReparacion>,
        vehiculo: String,
        descripcion: String,
        costo: u64,
    ) -> Result<()> {
        let reparacion = &mut ctx.accounts.reparacion;
        let usuario = &ctx.accounts.usuario;

        reparacion.cliente = *usuario.key;
        reparacion.vehiculo = vehiculo;
        reparacion.descripcion = descripcion;
        reparacion.costo = costo;
        reparacion.terminado = false;

        msg!("Reparación creada mediante PDA. Semilla: 'reparacion' + usuario");
        Ok(())
    }

    pub fn actualizar_costo(
        ctx: Context<ActualizarCosto>,
        nuevo_costo: u64,
    ) -> Result<()> {
        let reparacion = &mut ctx.accounts.reparacion;
        reparacion.costo = nuevo_costo;

        msg!("Costo actualizado a: {} lamports", nuevo_costo);
        Ok(())
    }

    pub fn terminar_reparacion(_ctx: Context<TerminarReparacion>) -> Result<()> {
        // El constraint 'close' hace todo el trabajo de devolver el SOL
        msg!("Reparación finalizada. Cuenta cerrada y renta devuelta.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearReparacion<'info> {
    #[account(
        init, 
        payer = usuario, 
        space = 8 + 32 + 44 + 104 + 8 + 1,
        // Agregamos las semillas para convertir la cuenta en un PDA
        seeds = [b"reparacion", usuario.key().as_ref()],
        bump
    )]
    pub reparacion: Account<'info, Reparacion>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarCosto<'info> {
    #[account(
        mut, 
        has_one = cliente,
        // Debemos incluir las semillas también aquí para localizar la cuenta
        seeds = [b"reparacion", cliente.key().as_ref()],
        bump
    )]
    pub reparacion: Account<'info, Reparacion>,
    
    pub cliente: Signer<'info>,
}

#[derive(Accounts)]
pub struct TerminarReparacion<'info> {
    #[account(
        mut, 
        has_one = cliente, 
        close = cliente,
        seeds = [b"reparacion", cliente.key().as_ref()],
        bump
    )]
    pub reparacion: Account<'info, Reparacion>,
    
    #[account(mut)]
    pub cliente: Signer<'info>,
}

#[account]
pub struct Reparacion {
    pub cliente: Pubkey,   
    pub vehiculo: String,  
    pub descripcion: String, 
    pub costo: u64,        
    pub terminado: bool,   
}