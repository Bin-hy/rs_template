import { request } from "@/http/axios"
import type * as Auth from "./type"

/** 登录并返回 Token */
export function loginApi(data: Auth.LoginRequestData) {
  return request<Auth.LoginResponseData>({
    url: "users/login", // auth/login
    method: "post",
    data
  })
}

export function testApi(){
    return request({
        url: "error",
        method: "get"
    })
}
