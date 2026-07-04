# Sylph  

```text
   _____       __      __  
  / ___/__  __/ /___  / /_ 
  \__ \/ / / / / __ \/ __ \
 ___/ / /_/ / / /_/ / / / /
/____/\__, /_/ .___/_/ /_/ 
     /____/ /_/            
```

## 说明

Sylph 是一个实验性的数据库, 用于验证我自己的一些架构设计想法, 还在开发中.  

本项目以学习和探索为主要目的, 未经严格的正确性检查与性能优化, 不适合在生产环境使用.

本项目采用 [MIT License](LICENSE) 进行许可.  

## 设计概览

### 核心流程

```mermaid
%%{init: { 
  "theme": "base",
  "themeVariables": {
    "primaryColor": "#c5e6b6",
    "primaryTextColor": "#1F3A1F",
    "primaryBorderColor": "#8FB78A",
    "edgeLabelBackground": "#C6D7DB",
    "lineColor": "#aad9f2"
  }
} }%%
flowchart TB
    A[用户查询] --> |分词| B[字符列表]
    B --> |ID映射| C[字符ID序列]
    C --> |合并成短语| D[词条集合]
    D --> |查倒排索引| E[各词条倒排列表]
    E --> |多路归并求交| F[文章 ID 列表]
    F --> |读取| G[返回文章内容]
```
