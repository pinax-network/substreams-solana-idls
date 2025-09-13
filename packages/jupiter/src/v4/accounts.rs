use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use idls_common::accounts;

// -----------------------------------------------------------------------------
// Route
// -----------------------------------------------------------------------------
accounts!(
    RouteAccounts,
    get_route_accounts,
    {
        token_program,
        user_transfer_authority,
        destination_token_account,
    }
);

// -----------------------------------------------------------------------------
// Whirlpoolswapexactoutput
// -----------------------------------------------------------------------------
accounts!(
    WhirlpoolswapexactoutputAccounts,
    get_whirlpool_swap_exact_output_accounts,
    {
        swap_program,
        token_program,
        token_authority,
        whirlpool,
        token_owner_account_a,
        token_vault_a,
        token_owner_account_b,
        token_vault_b,
        tick_array0,
        tick_array1,
        tick_array2,
        /// Oracle is currently unused and will be enabled on subsequent updates
        oracle,
    }
);

// -----------------------------------------------------------------------------
// Raydiumswapexactoutput
// -----------------------------------------------------------------------------
accounts!(
    RaydiumswapexactoutputAccounts,
    get_raydium_swap_exact_output_accounts,
    {
        swap_program,
        token_program,
        amm_id,
        amm_authority,
        amm_open_orders,
        pool_coin_token_account,
        pool_pc_token_account,
        serum_program_id,
        serum_market,
        serum_bids,
        serum_asks,
        serum_event_queue,
        serum_coin_vault_account,
        serum_pc_vault_account,
        serum_vault_signer,
        user_source_token_account,
        user_destination_token_account,
        user_source_owner,
    }
);

// -----------------------------------------------------------------------------
// Raydiumclmmswapexactoutput
// -----------------------------------------------------------------------------
accounts!(
    RaydiumclmmswapexactoutputAccounts,
    get_raydium_clmm_swap_exact_output_accounts,
    {
        swap_program,
        payer,
        amm_config,
        pool_state,
        input_token_account,
        output_token_account,
        input_vault,
        output_vault,
        observation_state,
        token_program,
        tick_array,
    }
);

// -----------------------------------------------------------------------------
// Createopenorders
// -----------------------------------------------------------------------------
accounts!(
    CreateopenordersAccounts,
    get_create_open_orders_accounts,
    {
        open_orders,
        payer,
        dex_program,
        system_program,
        rent,
        market,
    }
);

// -----------------------------------------------------------------------------
// Createopenorderv2
// -----------------------------------------------------------------------------
accounts!(
    Createopenorderv2Accounts,
    get_create_open_order_v2_accounts,
    {
        open_orders,
        payer,
        dex_program,
        system_program,
        rent,
        market,
    }
);

// -----------------------------------------------------------------------------
// Mercurialswap
// -----------------------------------------------------------------------------
accounts!(
    MercurialswapAccounts,
    get_mercurial_swap_accounts,
    {
        swap_program,
        swap_state,
        token_program,
        pool_authority,
        user_transfer_authority,
        source_token_account,
        destination_token_account,
    }
);

// -----------------------------------------------------------------------------
// Cykuraswap
// -----------------------------------------------------------------------------
accounts!(
    CykuraswapAccounts,
    get_cykura_swap_accounts,
    {
        swap_program,
        signer,
        factory_state,
        pool_state,
        input_token_account,
        output_token_account,
        input_vault,
        output_vault,
        last_observation_state,
        core_program,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Serumswap
// -----------------------------------------------------------------------------
accounts!(
    SerumswapAccounts,
    get_serum_swap_accounts,
    {
        market,
        authority,
        order_payer_token_account,
        coin_wallet,
        pc_wallet,
        dex_program,
        token_program,
        rent,
    }
);

// -----------------------------------------------------------------------------
// Saberswap
// -----------------------------------------------------------------------------
accounts!(
    SaberswapAccounts,
    get_saber_swap_accounts,
    {
        swap_program,
        token_program,
        swap,
        swap_authority,
        user_authority,
        input_user_account,
        input_token_account,
        output_user_account,
        output_token_account,
        fees_token_account,
    }
);

// -----------------------------------------------------------------------------
// Saberadddecimals
// -----------------------------------------------------------------------------
accounts!(
    SaberadddecimalsAccounts,
    get_saber_add_decimals_accounts,
    {
        add_decimals_program,
        wrapper,
        wrapper_mint,
        wrapper_underlying_tokens,
        owner,
        user_underlying_tokens,
        user_wrapped_tokens,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Tokenswap
// -----------------------------------------------------------------------------
accounts!(
    TokenswapAccounts,
    get_token_swap_accounts,
    {
        token_swap_program,
        token_program,
        swap,
        authority,
        user_transfer_authority,
        source,
        swap_source,
        swap_destination,
        destination,
        pool_mint,
        pool_fee,
    }
);

// -----------------------------------------------------------------------------
// Senchaswap
// -----------------------------------------------------------------------------
accounts!(
    SenchaswapAccounts,
    get_sencha_swap_accounts,
    {
        swap_program,
        token_program,
        swap,
        user_authority,
        input_user_account,
        input_token_account,
        input_fees_account,
        output_user_account,
        output_token_account,
        output_fees_account,
    }
);

// -----------------------------------------------------------------------------
// Stepswap
// -----------------------------------------------------------------------------
accounts!(
    StepswapAccounts,
    get_step_swap_accounts,
    {
        token_swap_program,
        token_program,
        swap,
        authority,
        user_transfer_authority,
        source,
        swap_source,
        swap_destination,
        destination,
        pool_mint,
        pool_fee,
    }
);

// -----------------------------------------------------------------------------
// Cropperswap
// -----------------------------------------------------------------------------
accounts!(
    CropperswapAccounts,
    get_cropper_swap_accounts,
    {
        token_swap_program,
        token_program,
        swap,
        swap_state,
        authority,
        user_transfer_authority,
        source,
        swap_source,
        swap_destination,
        destination,
        pool_mint,
        pool_fee,
    }
);

// -----------------------------------------------------------------------------
// Raydiumswap
// -----------------------------------------------------------------------------
accounts!(
    RaydiumswapAccounts,
    get_raydium_swap_accounts,
    {
        swap_program,
        token_program,
        amm_id,
        amm_authority,
        amm_open_orders,
        pool_coin_token_account,
        pool_pc_token_account,
        serum_program_id,
        serum_market,
        serum_bids,
        serum_asks,
        serum_event_queue,
        serum_coin_vault_account,
        serum_pc_vault_account,
        serum_vault_signer,
        user_source_token_account,
        user_destination_token_account,
        user_source_owner,
    }
);

// -----------------------------------------------------------------------------
// Cremaswap
// -----------------------------------------------------------------------------
accounts!(
    CremaswapAccounts,
    get_crema_swap_accounts,
    {
        swap_program,
        clmm_config,
        clmmpool,
        token_a,
        token_b,
        account_a,
        account_b,
        token_a_vault,
        token_b_vault,
        tick_array_map,
        owner,
        partner,
        partner_ata_a,
        partner_ata_b,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Lifinityswap
// -----------------------------------------------------------------------------
accounts!(
    LifinityswapAccounts,
    get_lifinity_swap_accounts,
    {
        swap_program,
        authority,
        amm,
        user_transfer_authority,
        source_info,
        destination_info,
        swap_source,
        swap_destination,
        pool_mint,
        fee_account,
        token_program,
        pyth_account,
        pyth_pc_account,
        config_account,
    }
);

// -----------------------------------------------------------------------------
// Marinadedeposit
// -----------------------------------------------------------------------------
accounts!(
    MarinadedepositAccounts,
    get_marinade_deposit_accounts,
    {
        marinade_finance_program,
        state,
        msol_mint,
        liq_pool_sol_leg_pda,
        liq_pool_msol_leg,
        liq_pool_msol_leg_authority,
        reserve_pda,
        transfer_from,
        mint_to,
        msol_mint_authority,
        system_program,
        token_program,
        user_wsol_token_account,
        temp_wsol_token_account,
        user_transfer_authority,
        wsol_mint,
        rent,
    }
);

// -----------------------------------------------------------------------------
// Marinadeunstake
// -----------------------------------------------------------------------------
accounts!(
    MarinadeunstakeAccounts,
    get_marinade_unstake_accounts,
    {
        marinade_finance_program,
        state,
        msol_mint,
        liq_pool_sol_leg_pda,
        liq_pool_msol_leg,
        treasury_msol_account,
        get_msol_from,
        get_msol_from_authority,
        transfer_sol_to,
        system_program,
        token_program,
        user_wsol_token_account,
    }
);

// -----------------------------------------------------------------------------
// Aldrinswap
// -----------------------------------------------------------------------------
accounts!(
    AldrinswapAccounts,
    get_aldrin_swap_accounts,
    {
        swap_program,
        pool,
        pool_signer,
        pool_mint,
        base_token_vault,
        quote_token_vault,
        fee_pool_token_account,
        wallet_authority,
        user_base_token_account,
        user_quote_token_account,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Aldrinv2swap
// -----------------------------------------------------------------------------
accounts!(
    Aldrinv2swapAccounts,
    get_aldrin_v2_swap_accounts,
    {
        swap_program,
        pool,
        pool_signer,
        pool_mint,
        base_token_vault,
        quote_token_vault,
        fee_pool_token_account,
        wallet_authority,
        user_base_token_account,
        user_quote_token_account,
        curve,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Whirlpoolswap
// -----------------------------------------------------------------------------
accounts!(
    WhirlpoolswapAccounts,
    get_whirlpool_swap_accounts,
    {
        swap_program,
        token_program,
        token_authority,
        whirlpool,
        token_owner_account_a,
        token_vault_a,
        token_owner_account_b,
        token_vault_b,
        tick_array0,
        tick_array1,
        tick_array2,
        /// Oracle is currently unused and will be enabled on subsequent updates
        oracle,
    }
);

// -----------------------------------------------------------------------------
// Invariantswap
// -----------------------------------------------------------------------------
accounts!(
    InvariantswapAccounts,
    get_invariant_swap_accounts,
    {
        swap_program,
        state,
        pool,
        tickmap,
        account_x,
        account_y,
        reserve_x,
        reserve_y,
        owner,
        program_authority,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Meteoraswap
// -----------------------------------------------------------------------------
accounts!(
    MeteoraswapAccounts,
    get_meteora_swap_accounts,
    {
        swap_program,
        pool,
        user_source_token,
        user_destination_token,
        a_vault,
        b_vault,
        a_token_vault,
        b_token_vault,
        a_vault_lp_mint,
        b_vault_lp_mint,
        a_vault_lp,
        b_vault_lp,
        admin_token_fee,
        user,
        vault_program,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Goosefxswap
// -----------------------------------------------------------------------------
accounts!(
    GoosefxswapAccounts,
    get_goosefx_swap_accounts,
    {
        swap_program,
        controller,
        pair,
        ssl_in,
        ssl_out,
        liability_vault_in,
        swapped_liability_vault_in,
        liability_vault_out,
        swapped_liability_vault_out,
        user_in_ata,
        user_out_ata,
        fee_collector_ata,
        user_wallet,
        fee_collector,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Deltafiswap
// -----------------------------------------------------------------------------
accounts!(
    DeltafiswapAccounts,
    get_deltafi_swap_accounts,
    {
        swap_program,
        market_config,
        swap_info,
        user_source_token,
        user_destination_token,
        swap_source_token,
        swap_destination_token,
        deltafi_user,
        admin_destination_token,
        pyth_price_base,
        pyth_price_quote,
        user_authority,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Balansolswap
// -----------------------------------------------------------------------------
accounts!(
    BalansolswapAccounts,
    get_balansol_swap_accounts,
    {
        swap_program,
        authority,
        pool,
        tax_man,
        bid_mint,
        treasurer,
        src_treasury,
        src_associated_token_account,
        ask_mint,
        dst_treasury,
        dst_associated_token_account,
        dst_token_account_taxman,
        system_program,
        token_program,
        associated_token_program,
        rent,
    }
);

// -----------------------------------------------------------------------------
// Marcopoloswap
// -----------------------------------------------------------------------------
accounts!(
    MarcopoloswapAccounts,
    get_marco_polo_swap_accounts,
    {
        swap_program,
        state,
        pool,
        token_x,
        token_y,
        pool_x_account,
        pool_y_account,
        swapper_x_account,
        swapper_y_account,
        swapper,
        referrer_x_account,
        referrer_y_account,
        referrer,
        program_authority,
        system_program,
        token_program,
        associated_token_program,
        rent,
    }
);

// -----------------------------------------------------------------------------
// Dradexswap
// -----------------------------------------------------------------------------
accounts!(
    DradexswapAccounts,
    get_dradex_swap_accounts,
    {
        swap_program,
        pair,
        market,
        event_queue,
        dex_user,
        market_user,
        bids,
        asks,
        t0_vault,
        t1_vault,
        t0_user,
        t1_user,
        master,
        signer,
        system_program,
        token_program,
        logger,
    }
);

// -----------------------------------------------------------------------------
// Lifinityv2swap
// -----------------------------------------------------------------------------
accounts!(
    Lifinityv2swapAccounts,
    get_lifinity_v2_swap_accounts,
    {
        swap_program,
        authority,
        amm,
        user_transfer_authority,
        source_info,
        destination_info,
        swap_source,
        swap_destination,
        pool_mint,
        fee_account,
        token_program,
        oracle_main_account,
        oracle_sub_account,
        oracle_pc_account,
    }
);

// -----------------------------------------------------------------------------
// Raydiumclmmswap
// -----------------------------------------------------------------------------
accounts!(
    RaydiumclmmswapAccounts,
    get_raydium_clmm_swap_accounts,
    {
        swap_program,
        payer,
        amm_config,
        pool_state,
        input_token_account,
        output_token_account,
        input_vault,
        output_vault,
        observation_state,
        token_program,
        tick_array,
    }
);

// -----------------------------------------------------------------------------
// Phoenixswap
// -----------------------------------------------------------------------------
accounts!(
    PhoenixswapAccounts,
    get_phoenix_swap_accounts,
    {
        swap_program,
        log_authority,
        market,
        trader,
        base_account,
        quote_account,
        base_vault,
        quote_vault,
        token_program,
    }
);

// -----------------------------------------------------------------------------
// Claimbonk
// -----------------------------------------------------------------------------
accounts!(
    ClaimbonkAccounts,
    get_claim_bonk_accounts,
    {
        open_orders,
        bonk_mint,
        open_orders_bonk_token_account,
        market,
        open_orders_owner,
        claimer_token_account,
        token_program,
    }
);
