use super::*;
/// File generated by fuel-core: benches/src/bin/collect.rs:440. With the following git
/// hash
pub const GIT: &str = "98341e564b75d1157e61d7d5f38612f6224a5b30";
pub fn default_gas_costs() -> GasCostsValues {
    GasCostsValues {
        add: 1,
        addi: 1,
        aloc: 1,
        and: 1,
        andi: 1,
        bal: 13,
        bhei: 1,
        bhsh: 1,
        burn: 132,
        cb: 1,
        cfei: 1,
        cfsi: 1,
        croo: 16,
        div: 1,
        divi: 1,
        eck1: 951,
        ecr1: 3000,
        ed19: 3000,
        eq: 1,
        exp: 1,
        expi: 1,
        flag: 1,
        gm: 1,
        gt: 1,
        gtf: 1,
        ji: 1,
        jmp: 1,
        jne: 1,
        jnei: 1,
        jnzi: 1,
        jmpf: 1,
        jmpb: 1,
        jnzf: 1,
        jnzb: 1,
        jnef: 1,
        jneb: 1,
        lb: 1,
        log: 9,
        lt: 1,
        lw: 1,
        mint: 135,
        mlog: 1,
        modi: 1,
        mod_op: 1,
        movi: 1,
        mroo: 2,
        mul: 1,
        muli: 1,
        mldv: 1,
        noop: 1,
        not: 1,
        or: 1,
        ori: 1,
        poph: 2,
        popl: 2,
        pshh: 2,
        pshl: 2,
        move_op: 1,
        ret: 13,
        sb: 1,
        sll: 1,
        slli: 1,
        srl: 1,
        srli: 1,
        srw: 12,
        sub: 1,
        subi: 1,
        sw: 1,
        sww: 67,
        time: 1,
        tr: 105,
        tro: 60,
        wdcm: 1,
        wqcm: 1,
        wdop: 1,
        wqop: 1,
        wdml: 1,
        wqml: 1,
        wddv: 1,
        wqdv: 2,
        wdmd: 3,
        wqmd: 4,
        wdam: 2,
        wqam: 3,
        wdmm: 3,
        wqmm: 3,
        xor: 1,
        xori: 1,
        k256: DependentCost {
            base: 11,
            dep_per_unit: 214,
        },
        s256: DependentCost {
            base: 2,
            dep_per_unit: 214,
        },
        call: DependentCost {
            base: 144,
            dep_per_unit: 214,
        },
        ccp: DependentCost {
            base: 15,
            dep_per_unit: 103,
        },
        csiz: DependentCost {
            base: 17,
            dep_per_unit: 790,
        },
        ldc: DependentCost {
            base: 15,
            dep_per_unit: 272,
        },
        logd: DependentCost {
            base: 26,
            dep_per_unit: 64,
        },
        mcl: DependentCost {
            base: 1,
            dep_per_unit: 3333,
        },
        mcli: DependentCost {
            base: 1,
            dep_per_unit: 3333,
        },
        mcp: DependentCost {
            base: 1,
            dep_per_unit: 2000,
        },
        mcpi: DependentCost {
            base: 3,
            dep_per_unit: 2000,
        },
        meq: DependentCost {
            base: 1,
            dep_per_unit: 2500,
        },
        rvrt: 13,
        smo: DependentCost {
            base: 209,
            dep_per_unit: 55,
        },
        retd: DependentCost {
            base: 29,
            dep_per_unit: 62,
        },
        srwq: DependentCost {
            base: 47,
            dep_per_unit: 5,
        },
        scwq: DependentCost {
            base: 13,
            dep_per_unit: 5,
        },
        swwq: DependentCost {
            base: 44,
            dep_per_unit: 5,
        },
        contract_root: DependentCost {
            base: 75,
            dep_per_unit: 1,
        },
    }
}
