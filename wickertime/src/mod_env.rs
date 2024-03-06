use cranelift_wasm::ModuleEnvironment;

pub struct ModuleEnv {}

impl ModuleEnvironment for ModuleEnv {
    fn declare_type_func(
        &mut self,
        wasm_func_type: cranelift_wasm::WasmFuncType,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_func_import(
        &mut self,
        index: cranelift_wasm::TypeIndex,
        module: &'data str,
        field: &'data str,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_table_import(
        &mut self,
        table: cranelift_wasm::Table,
        module: &'data str,
        field: &'data str,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_memory_import(
        &mut self,
        memory: cranelift_wasm::Memory,
        module: &'data str,
        field: &'data str,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_global_import(
        &mut self,
        global: cranelift_wasm::Global,
        module: &'data str,
        field: &'data str,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_func_type(
        &mut self,
        index: cranelift_wasm::TypeIndex,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_table(&mut self, table: cranelift_wasm::Table) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_memory(&mut self, memory: cranelift_wasm::Memory) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_global(
        &mut self,
        global: cranelift_wasm::Global,
        init: cranelift_wasm::GlobalInit,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_func_export(
        &mut self,
        func_index: cranelift_wasm::FuncIndex,
        name: &'data str,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_table_export(
        &mut self,
        table_index: cranelift_wasm::TableIndex,
        name: &'data str,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_memory_export(
        &mut self,
        memory_index: cranelift_wasm::MemoryIndex,
        name: &'data str,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_global_export(
        &mut self,
        global_index: cranelift_wasm::GlobalIndex,
        name: &'data str,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_start_func(
        &mut self,
        index: cranelift_wasm::FuncIndex,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_table_elements(
        &mut self,
        table_index: cranelift_wasm::TableIndex,
        base: Option<cranelift_wasm::GlobalIndex>,
        offset: u32,
        elements: Box<[cranelift_wasm::FuncIndex]>,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_passive_element(
        &mut self,
        index: cranelift_wasm::ElemIndex,
        elements: Box<[cranelift_wasm::FuncIndex]>,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_passive_data(
        &mut self,
        data_index: cranelift_wasm::DataIndex,
        data: &'data [u8],
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn define_function_body(
        &mut self,
        validator: cranelift_wasm::wasmparser::FuncValidator<
            cranelift_wasm::wasmparser::ValidatorResources,
        >,
        body: cranelift_wasm::wasmparser::FunctionBody<'data>,
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }

    fn declare_data_initialization(
        &mut self,
        memory_index: cranelift_wasm::MemoryIndex,
        base: Option<cranelift_wasm::GlobalIndex>,
        offset: u64,
        data: &'data [u8],
    ) -> cranelift_wasm::WasmResult<()> {
        todo!()
    }
}
