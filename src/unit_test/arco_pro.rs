use crate::extend::string::StringExtend;
use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;

#[test]
fn test_arco_pro_1_less() {
  let filepath = path_resolve("assets/arco-pro/1.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.customer-tooltip-title {
  margin-bottom: 4px;
}
.customer-tooltip-item {
  height: 32px;
  line-height: 32px;
  display: flex;
  justify-content: space-between;
  padding: 0 8px;
  background: rgba(255, 255, 255, 0.9);
  box-shadow: 6px 0 20px rgba(34, 87, 188, 0.1);
  border-radius: 4px;
  color: var(--color-text-2);
}
.customer-tooltip-item :global(.arco-badge-status-dot) {
  width: 10px;
  height: 10px;
  margin-right: 8px;
}
.customer-tooltip-item:not(:last-child) {
  margin-bottom: 8px;
}
body[arco-theme='dark'] .customer-tooltip-item {
  background: #2a2a2b;
  box-shadow: 6px 0px 20px rgba(34, 87, 188, 0.1);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_2_less() {
  let filepath = path_resolve("assets/arco-pro/2.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.footer{
  display: flex;
  align-items: center;
  justify-content: center;
  height: 40px;
  text-align: center;
  color: var(--color-text-2);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_3_less() {
  let filepath = path_resolve("assets/arco-pro/3.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.message-box{
  width: 400px;
  max-height: 800px;
  background-color: var(--color-bg-popup);
  border: 1px solid var(--color-border-2);
  box-shadow: 0 4px 10px rgba(0 , 0 , 0 , 0.1);
  border-radius: 4px;

}

.message-box :global(.arco-tabs-header-nav){
  padding: 8px 16px;
  border-bottom: 1px solid var(--color-border-2);

}

.message-box :global(.arco-list-item-meta){
  align-items: flex-start;

}

.message-box :global(.arco-list-item-meta-content){
  width: 100%;

}

.message-box :global(.arco-tabs-content){
  padding-top: 0;

}

.message-title{
  display: flex;
  justify-content: space-between;

}

.footer{
  display: flex;

}

.footer-item{
  display: flex;
  justify-content: center;
  width: 50%;

}

.footer-item:first-child{
  border-right: 1px solid var(--color-border-2);

}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_4_less() {
  let filepath = path_resolve("assets/arco-pro/4.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.icon-button{
  font-size: 16px;
  border: 1px solid var(--color-border-2);
}

.icon-button > svg{
  vertical-align: -3px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_5_less() {
  let filepath = path_resolve("assets/arco-pro/5.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.navbar {
  display: flex;
  justify-content: space-between;
  border-bottom: 1px solid var(--color-border);
  box-sizing: border-box;
  background-color: var(--color-bg-2);
  height: 100%;
}
.left {
  display: flex;
  align-items: center;
}
.logo {
  display: flex;
  align-items: center;
  width: 200px;
  padding-left: 20px;
  box-sizing: border-box;
}
.logo-name {
  color: var(--color-text-1);
  font-weight: 500;
  font-size: 20px;
  margin-left: 10px;
  font-family: 'PingFang SC';
}
.right {
  display: flex;
  list-style: none;
  padding-right: 20px;
}
.right li {
  padding: 0 8px;
  display: flex;
  align-items: center;
}
.right a {
  text-decoration: none;
  color: var(--color-text-1);
}
.username {
  cursor: pointer;
}
.round :global(.arco-input-inner-wrapper) {
  border-radius: 16px;
}
.round svg {
  font-size: 16px;
}
.dropdown-icon {
  margin-right: 8px;
  font-size: 16px;
  vertical-align: text-bottom;
}
.fixed-settings {
  position: fixed;
  top: 280px;
  right: 0px;
}
.fixed-settings svg {
  font-size: 18px;
  vertical-align: -4px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_6_less() {
  let filepath = path_resolve("assets/arco-pro/6.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.panel {
  background-color: var(--color-bg-2);
  border-radius: 4px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_7_less() {
  let filepath = path_resolve("assets/arco-pro/7.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.block {
  margin-bottom: 24px;
}
.title {
  font-size: 14px;
  padding: 0;
  margin: 10px 0;
}
.switch-wrapper {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 32px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_8_less() {
  let filepath = path_resolve("assets/arco-pro/8.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.input {
  display: flex;
  width: 100%;
  height: 32px;
  border: 1px solid var(--color-border);
  padding: 3px;
  box-sizing: border-box;
}
.color {
  width: 100px;
  height: 24px;
  margin-right: 10px;
}
.ul {
  list-style: none;
  display: flex;
  padding: 0;
}
.li {
  width: 10%;
  height: 26px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_9_less() {
  let filepath = path_resolve("assets/arco-pro/9.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.message-item {
  font-size: 12px;
  line-height: 20px;
  padding: 8px;
  border-radius: 2px;
}
.message-item-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.message-item-actions {
  display: flex;
  opacity: 0;
}
.message-item-actions-item {
  margin-right: 4px;
  font-size: 14px;
  color: var(--color-text-3);
  width: 20px;
  height: 20px;
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}
.message-item-actions-item:hover {
  background-color: rgb(var(--gray-3));
}
.message-item-actions-item:last-child {
  margin-right: 0;
}
.message-item-collected .message-item-actions-collect {
  color: rgb(var(--gold-6));
}
.message-item:hover {
  background-color: rgb(var(--gray-2));
}
.message-item:hover .message-item-actions {
  opacity: 1;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_10_less() {
  let filepath = path_resolve("assets/arco-pro/10.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.layout {
  display: flex;
}
.layout-left-side {
  flex-basis: 300px;
}
.layout-content {
  flex: 1;
  padding: 0 16px;
}
.layout-right-side {
  flex-basis: 280px;
}
.chat-panel {
  height: 100%;
  background-color: var(--color-bg-2);
  padding: 20px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  border-radius: 4px;
}
.chat-panel-content {
  flex: 1;
  margin: 20px 0;
  box-sizing: border-box;
}
.data-statistic-content {
  padding: 20px 0;
}
.data-statistic-list-header {
  margin-top: 16px;
  display: flex;
  justify-content: space-between;
}
.data-statistic-list-content {
  margin-top: 16px;
}
.data-statistic-list-cover-wrapper {
  height: 68px;
  position: relative;
}
.data-statistic-list-cover-wrapper img {
  height: 100%;
}
.data-statistic-list-cover-tag {
  position: absolute;
  top: 6px;
  left: 6px;
}
.data-statistic-list-tip {
  display: block;
  margin-top: 16px;
  text-align: center;
}
.studio-wrapper :global(.arco-card-body) {
  padding-top: 0 !important;
}
.studio-preview {
  width: 100%;
  max-width: 600px;
  display: block;
  margin: 0 auto;
}
.studio-bar {
  margin-top: 16px;
  display: flex;
  justify-content: space-between;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_11_less() {
  let filepath = path_resolve("assets/arco-pro/11.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.item {
  display: flex;
  align-items: center;
  width: 100%;
  height: 24px;
  margin-bottom: 4px;
}
.link {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-left: 4px;
  color: var(--color-text-2);
  text-decoration: none;
  font-size: 13px;
  cursor: pointer;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_12_less() {
  let filepath = path_resolve("assets/arco-pro/12.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.docs {
  display: grid;
  grid-template-columns: 50% 50%;
}
.link {
  color: var(--color-text-2);
  padding: 4px;
  box-sizing: border-box;
  margin-bottom: 12px;
}
.link:hover {
  color: rgb(var(--primary-6));
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_13_less() {
  let filepath = path_resolve("assets/arco-pro/13.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.banner {
  background-color: var(--color-bg-2);
  padding: 20px;
}
.content {
  padding: 20px;
  display: flex;
}
.right {
  width: 280px;
}
.panel {
  background-color: var(--color-bg-2);
  border-radius: 4px;
  overflow: auto;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_14_less() {
  let filepath = path_resolve("assets/arco-pro/14.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.container {
  padding: 20px;
}
.container :global(.arco-divider-horizontal) {
  border-bottom: 1px solid var(--color-border-1);
}
.container :global(.arco-divider-vertical) {
  border-left: 1px solid var(--color-border-1);
}
.item {
  display: flex;
  align-items: center;
  padding-left: 20px;
  color: var(--color-text-1);
}
.icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 54px;
  height: 54px;
  background-color: var(--color-fill-2);
  border-radius: 50%;
  margin-right: 12px;
}
.title {
  font-size: 12px;
  color: var(--color-text-1);
}
.count {
  font-size: 22px;
  font-weight: 600;
  color: var(--color-text-1);
}
.count .unit {
  font-size: 12px;
  font-weight: 400;
  color: var(--color-text-2);
  margin-left: 8px;
}
.divider {
  height: 60px;
}
.ctw {
  display: flex;
  justify-content: space-between;
  margin-bottom: 16px;
}
.chart-title {
  font-size: 16px;
  font-weight: 500;
}
.chart-sub-title {
  font-size: 12px;
  font-weight: 400;
  margin-left: 4px;
  color: var(--color-text-3);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_15_less() {
  let filepath = path_resolve("assets/arco-pro/15.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.symbol {
  font-size: 10px;
  margin-left: 4px;
}
.symbol > svg {
  vertical-align: 0;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_16_less() {
  let filepath = path_resolve("assets/arco-pro/16.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.shortcuts {
  display: grid;
  grid-template-columns: 33.33% 33.33% 33.33%;
}
.item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 12px;
  box-sizing: border-box;
  cursor: pointer;
}
.item:hover .icon {
  background-color: var(--color-primary-light-1);
}
.item:hover .icon svg {
  color: rgb(var(--primary-6));
}
.item:hover .title {
  color: rgb(var(--primary-6));
}
.icon {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 32px;
  height: 32px;
  border-radius: 6px;
  background-color: var(--color-fill-2);
  margin-bottom: 4px;
}
.icon svg {
  font-size: 18px;
}
.title {
  font-size: 12px;
  line-height: 20px;
  color: var(--color-text-1);
}
.recent {
  font-weight: 500;
  font-size: 16px;
  line-height: 24px;
  color: var(--color-text-1);
  margin-bottom: 16px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_17_less() {
  let filepath = path_resolve("assets/arco-pro/17.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.wrapper {
  position: relative;
  background-color: var(--color-bg-1);
  height: calc(100vh - 168px);
}
.result {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_18_less() {
  let filepath = path_resolve("assets/arco-pro/18.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.wrapper {
  position: relative;
  background-color: var(--color-bg-1);
  height: calc(100vh - 168px);
}
.result {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_19_less() {
  let filepath = path_resolve("assets/arco-pro/19.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.wrapper {
  position: relative;
  background-color: var(--color-bg-1);
  height: calc(100vh - 168px);
}
.result {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}


#[test]
fn test_arco_pro_20_less() {
  let filepath = path_resolve("assets/arco-pro/20.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.container {
  overflow: hidden;
}
.container :global(.arco-card-body) {
  padding: 20px 20px 10px;
}
.container :global(.arco-card) {
  margin-bottom: 16px;
}
.actions {
  padding: 12px 40px;
  background-color: var(--color-bg-2);
  display: flex;
  flex-direction: row-reverse;
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  box-shadow: 0 -3px 12px rgba(0, 0, 0, 0.1);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

