# Todolist API

## 起動方法
```sh
docker compose up
```
環境変数は`compose.yml`内に適当な値を入れてあるので設定する必要はありません

## Endpoints
### `/auth/register`
#### POST
ユーザーを登録します。

##### リクエストボディの形式
```ts
type Request = {
  name: string; // ^[a-zA-Z0-9][a-zA-Z0-9 _-]{0,18}[a-zA-Z0-9]$
  password: string; // ^[a-zA-Z0-9!@#$%_]{8,20}$
};
```

##### 実行例
```sh
curl -i http://localhost:3000/auth/register -X POST -H "Content-Type: application/json" -d '{ "name":"hoge", "password":"hogehoge" }'
```

### `/auth/login`
#### POST
認証を試みます。
成功した場合JWTトークンが返ります。

##### リクエストボディの形式
```ts
type Request = {
  name: string; // ^[a-zA-Z0-9][a-zA-Z0-9 _-]{0,18}[a-zA-Z0-9]$
  password: string; // ^[a-zA-Z0-9!@#$%_]{8,20}$
};
```
##### レスポンスの形式
```ts
type Response = {
  token: string // JSON Web token
};
```

##### 実行例
```sh
curl -i http://localhost:3000/auth/login -X POST -H "Content-Type: application/json" -d '{ "name":"hoge", "password":"hogehoge" }'
```

### `/todo/task`
#### POST
タスクの登録をします。

##### リクエストボディの形式
```ts
type Request = {
  title: string; // 1文字以上100文字以下
  description: string; // 1文字以上1000文字以下
  status: "todo" | "doing" | "done";
  deadline: string | null; // ISO8601(UTC)
};
```

##### 実行例
```sh
curl -i http://localhost:3000/todo/task -X POST -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN" -d '{ "title":"旅行", "description":"行けたらいく", "status":"todo" }'
```

### `/todo/task/{id}`
#### GET
パスパラメータで指定されているIDのタスクの情報を取得します。

##### レスポンスの形式
```ts
type Response = {
  id: number;
  title: string; // 1文字以上100文字以下
  description: string; // 1文字以上1000文字以下
  status: "todo" | "doing" | "done";
  created_at: string; // ISO8601(UTC)
  updated_at: string | null; // ISO8601(UTC)
  deadline: string | null; // ISO8601(UTC)
};
```

##### 実行例
```sh
curl -i http://localhost:3000/todo/task/1 -X GET -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN"
```

#### PATCH
パスパラメータで指定されているIDのタスクが置き換えられます。

##### リクエストボディの形式
```ts
type Request = {
  title: string; // 1文字以上100文字以下
  description: string; // 1文字以上1000文字以下
  status: "todo" | "doing" | "done";
  deadline: string | null; // ISO8601(UTC)
};
```

##### 実行例
```sh
curl -i http://localhost:3000/todo/task/1 -X PATCH -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN" -d '{ "title":"旅行", "description":"行けたらいく", "status":"doing" }'
```

#### DELETE
パスパラメータで指定されているIDのタスクが削除されます。

##### 実行例
```sh
curl -i http://localhost:3000/todo/task/1 -X DELETE -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN"
```

### `/todo/tasks`
#### GET
タスク一覧を取得します。

##### レスポンスの形式
```ts
type Task = {
  id: number;
  title: string; // 1文字以上100文字以下
  description: string; // 1文字以上1000文字以下
  status: "todo" | "doing" | "done";
  created_at: string; // ISO8601(UTC)
  updated_at: string | null; // ISO8601(UTC)
  deadline: string | null; // ISO8601(UTC)
};

type Response = Task[];
```

##### 実行例
```sh
curl -i http://localhost:3000/todo/tasks -X GET -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN"
```

### `/todo/tasks/{status}`
#### GET
パスパラメータで指定した状態のタスクのみ取得します。

```ts
type Task = {
  id: number;
  title: string; // 1文字以上100文字以下
  description: string; // 1文字以上1000文字以下
  status: "todo" | "doing" | "done";
  created_at: string; // ISO8601(UTC)
  updated_at: string | null; // ISO8601(UTC)
  deadline: string | null; // ISO8601(UTC)
};

type Response = Task[];
```

##### 実行例
```sh
curl -i http://localhost:3000/todo/tasks/todo -X GET -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN"
```

### `/todo/overdue`
#### GET
期限超過のタスクのみ取得します。

```ts
type Task = {
  id: number;
  title: string; // 1文字以上100文字以下
  description: string; // 1文字以上1000文字以下
  status: "todo" | "doing" | "done";
  created_at: string; // ISO8601(UTC)
  updated_at: string | null; // ISO8601(UTC)
  deadline: string | null; // ISO8601(UTC)
};

type Response = Task[];
```

##### 実行例
```sh
curl -i http://localhost:3000/todo/overdue -X GET -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN"
```