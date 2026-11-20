//! RED design probe: public SeaORM transaction failures must not be swallowed.
//!
//! These tests are deliberately failing with SeaORM 1.x ProxyDatabaseTrait,
//! whose begin/commit/rollback callbacks return (). Keep this PR in Draft
//! until a reviewed error-bearing public transaction interface is available.
//! They do NOT exercise a live CUBRID server.

use async_trait::async_trait;
use cubrid_protocol::value::Value as CubridValue;
use sea_orm::TransactionTrait;
use sea_orm_cubrid::{connect_with_client, CubridClient};

#[derive(Debug)]
struct FaultClient {
    fail: &'static str,
}

#[async_trait]
impl CubridClient for FaultClient {
    async fn query(
        &mut self,
        _sql: &str,
        _params: &[CubridValue],
    ) -> Result<cubrid_tokio::QueryResult, cubrid_tokio::Error> {
        Ok(cubrid_tokio::QueryResult {
            columns: vec![],
            rows: vec![],
            total_count: 0,
        })
    }

    async fn execute(
        &mut self,
        sql: &str,
        _params: &[CubridValue],
    ) -> Result<u64, cubrid_tokio::Error> {
        if self.fail == "begin" && sql == "BEGIN" {
            return Err(cubrid_tokio::Error::ConnectionClosed);
        }
        Ok(0)
    }

    async fn commit(&mut self) -> Result<(), cubrid_tokio::Error> {
        if self.fail == "commit" {
            return Err(cubrid_tokio::Error::ConnectionClosed);
        }
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), cubrid_tokio::Error> {
        if self.fail == "rollback" {
            return Err(cubrid_tokio::Error::ConnectionClosed);
        }
        Ok(())
    }

    async fn ping(&mut self) -> Result<String, cubrid_tokio::Error> {
        Ok("test-fault-client".into())
    }
}

#[tokio::test]
async fn a_failed_underlying_begin_must_be_visible_to_seaorm_caller() {
    let db = connect_with_client(FaultClient { fail: "begin" })
        .await
        .expect("proxy setup must succeed before transaction start");

    assert!(
        db.begin().await.is_err(),
        "SeaORM begin() returned success after the underlying CUBRID BEGIN failed"
    );
}

#[tokio::test]
async fn a_failed_underlying_commit_must_be_visible_to_seaorm_caller() {
    let db = connect_with_client(FaultClient { fail: "commit" })
        .await
        .expect("proxy setup must succeed");
    let tx = db.begin().await.expect("begin succeeds in this fixture");

    assert!(
        tx.commit().await.is_err(),
        "SeaORM commit() returned success after the CUBRID client commit failed"
    );
}

#[tokio::test]
async fn a_failed_underlying_rollback_must_be_visible_to_seaorm_caller() {
    let db = connect_with_client(FaultClient { fail: "rollback" })
        .await
        .expect("proxy setup must succeed");
    let tx = db.begin().await.expect("begin succeeds in this fixture");

    assert!(
        tx.rollback().await.is_err(),
        "SeaORM rollback() returned success after the CUBRID client rollback failed"
    );
}
