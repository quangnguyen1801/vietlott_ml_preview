use anyhow::Ok;
use async_trait::async_trait;
use model::{
    models::{number_frequency::NumberFrequency, number_frequency655::NumberFrequency655},
    modelviews::{
        number_frequency655_view::NumberFrequency655View,
        number_frequency_view::NumberFrequencyView,
    },
    share::mapping_object::{MappingObject, NumberFrequencyViewVariant},
};
use repository::{
    irepositories::{
        inumber_frequency655_repository::INumberFrequency655Repository,
        inumber_frequency_repository::INumberFrequencyRepository,
    },
    repositories::{
        number_frequency655_repository::NumberFrequency655Repository,
        number_frequency_repository::NumberFrequencyRepository,
    },
    shares::irepository::IRepository,
};

use crate::iservices::inumber_frequency_service::INumberFrequencyService;

pub struct NumberFrequencyService;
#[async_trait]
impl INumberFrequencyService for NumberFrequencyService {
    async fn fn_ser_get_by_id(id: i32, mode: String) -> anyhow::Result<NumberFrequencyViewVariant> {
        if mode == "645" {
            let number_frequency = NumberFrequencyRepository::fn_repo_get_by_id(id).await?;
            let json_str = serde_json::to_string(&number_frequency).unwrap();
            Ok(NumberFrequencyViewVariant::D645(
                NumberFrequencyView::map_a_to_b(&json_str),
            ))
        } else {
            let number_frequency = NumberFrequency655Repository::fn_repo_get_by_id(id).await?;
            let json_str = serde_json::to_string(&number_frequency).unwrap();
            Ok(NumberFrequencyViewVariant::D655(
                NumberFrequency655View::map_a_to_b(&json_str),
            ))
        }
    }

    async fn fn_ser_get_all(mode: String) -> anyhow::Result<Vec<NumberFrequencyViewVariant>> {
        if mode == "645" {
            let number_frequencies = NumberFrequencyRepository::fn_repo_get_all().await?;
            let mut result = Vec::new();
            for number_frequency in number_frequencies {
                let json_str = serde_json::to_string(&number_frequency).unwrap();
                result.push(NumberFrequencyViewVariant::D645(
                    NumberFrequencyView::map_a_to_b(&json_str),
                ));
            }
            Ok(result)
        } else {
            let number_frequencies = NumberFrequency655Repository::fn_repo_get_all().await?;
            let mut result = Vec::new();
            for number_frequency in number_frequencies {
                let json_str = serde_json::to_string(&number_frequency).unwrap();
                result.push(NumberFrequencyViewVariant::D655(
                    NumberFrequency655View::map_a_to_b(&json_str),
                ));
            }
            Ok(result)
        }
    }

    async fn fn_ser_get_by_pagination(
        mut page_indx: usize,
        mut page_size: usize,
        mode: String,
    ) -> anyhow::Result<Vec<NumberFrequencyViewVariant>> {
        let number_frequencies = NumberFrequencyService::fn_ser_get_all(mode).await?;
        if page_indx <= 0 {
            page_indx = 1;
        }
        if page_size <= 0 {
            page_size = 10;
        }
        let start_index = (page_indx - 1) * page_size;
        let end_index = page_indx * page_size;
        let number_frequency_in_page = if number_frequencies.len() > start_index {
            if number_frequencies.len() < end_index {
                &number_frequencies[start_index..]
            } else {
                &number_frequencies[start_index..end_index]
            }
        } else {
            &[]
        };
        return Ok(number_frequency_in_page.to_vec());
    }

    async fn fn_ser_create(
        obj: NumberFrequencyViewVariant,
    ) -> anyhow::Result<NumberFrequencyViewVariant> {
        match obj {
            NumberFrequencyViewVariant::D645(n) => {
                let number_frequency_json = serde_json::to_string(&n).unwrap();
                let number_frequency = NumberFrequency::map_a_to_b(&number_frequency_json);
                let result = NumberFrequencyRepository::fn_repo_create(number_frequency).await?;
                Ok(NumberFrequencyViewVariant::D645(
                    NumberFrequencyView::map_a_to_b(&serde_json::to_string(&result).unwrap()),
                ))
            }
            NumberFrequencyViewVariant::D655(n) => {
                let number_frequency_json = serde_json::to_string(&n).unwrap();
                let number_frequency = NumberFrequency655::map_a_to_b(&number_frequency_json);
                let result = NumberFrequency655Repository::fn_repo_create(number_frequency).await?;
                Ok(NumberFrequencyViewVariant::D655(
                    NumberFrequency655View::map_a_to_b(&serde_json::to_string(&result).unwrap()),
                ))
            }
        }
    }

    async fn fn_ser_update(
        obj: NumberFrequencyViewVariant,
    ) -> anyhow::Result<NumberFrequencyViewVariant> {
        match obj {
            NumberFrequencyViewVariant::D645(n) => {
                let number_frequency_json = serde_json::to_string(&n).unwrap();
                let number_frequency = NumberFrequency::map_a_to_b(&number_frequency_json);
                let result = NumberFrequencyRepository::fn_repo_update(number_frequency).await?;
                Ok(NumberFrequencyViewVariant::D645(
                    NumberFrequencyView::map_a_to_b(&serde_json::to_string(&result).unwrap()),
                ))
            }
            NumberFrequencyViewVariant::D655(n) => {
                let number_frequency_json = serde_json::to_string(&n).unwrap();
                let number_frequency = NumberFrequency655::map_a_to_b(&number_frequency_json);
                let result = NumberFrequency655Repository::fn_repo_update(number_frequency).await?;
                Ok(NumberFrequencyViewVariant::D655(
                    NumberFrequency655View::map_a_to_b(&serde_json::to_string(&result).unwrap()),
                ))
            }
        }
    }

    async fn fn_ser_delete(id: i32, mode: String) -> anyhow::Result<bool> {
        if mode == "645" {
            let result = NumberFrequencyRepository::fn_repo_delete(id).await?;
            Ok(result)
        } else {
            let result = NumberFrequency655Repository::fn_repo_delete(id).await?;
            Ok(result)
        }
    }

    async fn fn_ser_add_frequency_features_by_storeprocedures(
        drawid: i32,
        mode: String,
    ) -> anyhow::Result<Vec<NumberFrequencyViewVariant>> {
        if mode == "645" {
            let result_exec =
                NumberFrequencyRepository::fn_repo_execute_storeprocedures(drawid).await?;
            let mut result = Vec::new();
            for rc in result_exec {
                let json = serde_json::to_string(&rc)?;
                result.push(NumberFrequencyViewVariant::D645(
                    NumberFrequencyView::map_a_to_b(&json),
                ));
            }
            Ok(result)
        } else {
            let result_exec =
                NumberFrequency655Repository::fn_repo_execute_storeprocedures(drawid).await?;
            let mut result = Vec::new();
            for rc in result_exec {
                let json = serde_json::to_string(&rc)?;
                result.push(NumberFrequencyViewVariant::D655(
                    NumberFrequency655View::map_a_to_b(&json),
                ));
            }
            Ok(result)
        }
    }

    async fn fn_ser_get_frequency_by_drawid(
        drawid: i32,
        mode: String,
    ) -> anyhow::Result<Vec<NumberFrequencyViewVariant>> {
        if mode == "645" {
            let data = NumberFrequencyRepository::fn_repo_get_by_drawid(drawid).await?;
            let mut result = Vec::new();
            for numfreq in data {
                let json = serde_json::to_string(&numfreq)?;
                result.push(NumberFrequencyViewVariant::D645(
                    NumberFrequencyView::map_a_to_b(&json),
                ));
            }
            Ok(result)
        } else {
            let data = NumberFrequency655Repository::fn_repo_get_by_drawid(drawid).await?;
            let mut result = Vec::new();
            for numfreq in data {
                let json = serde_json::to_string(&numfreq)?;
                result.push(NumberFrequencyViewVariant::D655(
                    NumberFrequency655View::map_a_to_b(&json),
                ));
            }
            Ok(result)
        }
    }
}
