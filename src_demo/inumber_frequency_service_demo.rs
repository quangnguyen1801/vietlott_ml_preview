use async_trait::async_trait;
use model::share::mapping_object::NumberFrequencyViewVariant;
#[async_trait]
pub trait INumberFrequencyService {
    async fn fn_ser_get_by_id(id: i32, mode: String) -> anyhow::Result<NumberFrequencyViewVariant>;
    async fn fn_ser_get_all(mode: String) -> anyhow::Result<Vec<NumberFrequencyViewVariant>>;
    async fn fn_ser_get_by_pagination(
        mut page_indx: usize,
        mut page_size: usize,
        mode: String,
    ) -> anyhow::Result<Vec<NumberFrequencyViewVariant>>;
    async fn fn_ser_create(
        obj: NumberFrequencyViewVariant,
    ) -> anyhow::Result<NumberFrequencyViewVariant>;
    async fn fn_ser_update(
        obj: NumberFrequencyViewVariant,
    ) -> anyhow::Result<NumberFrequencyViewVariant>;
    async fn fn_ser_delete(id: i32, mode: String) -> anyhow::Result<bool>;
    async fn fn_ser_add_frequency_features_by_storeprocedures(
        drawid: i32,
        mode: String,
    ) -> anyhow::Result<Vec<NumberFrequencyViewVariant>>;
    async fn fn_ser_get_frequency_by_drawid(
        drawid: i32,
        mode: String,
    ) -> anyhow::Result<Vec<NumberFrequencyViewVariant>>;
}
