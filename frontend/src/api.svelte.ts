/*
 * Copyright (C) 2026 Leif Barton
 * Licensed under the Open Software License 3.0
 */

import { API_URL } from "./config";

let isHealthy: undefined | boolean = $state(undefined);

async function apiFetch(path: string, init: RequestInit = {}) {
  const url = API_URL + path;
  return await fetch(url, {
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    ...init,
  });
}

async function isApiReallyHealthy() {
  try {
    const response = await apiFetch("");
    return response.ok && (await response.text()) === "hello!";
  } catch (e) {
    console.error(e);
    return false;
  }
}

export async function isApiHealthy() {
  if (isHealthy === undefined) {
    console.log("pinging");
    const ret = await isApiReallyHealthy();
    isHealthy = ret;
    return ret;
  } else {
    return isHealthy;
  }
}

export async function createUser(
  username: string,
  password: string,
): Promise<Response> {
  return await apiFetch("newuser", {
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
}

export async function logIn(
  username: string,
  password: string,
): Promise<Response> {
  return await apiFetch("login", {
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
}

export interface MyAccount {
  id: string;
  username: string;
  permission: PermissionLevel;
}

export enum PermissionLevel {
  NotYetVerified = 20,
  Verified,
  Helper,
  Admin,
}

export const PermissionLevelLabel: Record<PermissionLevel, string> = {
  [PermissionLevel.NotYetVerified]: "Not Yet Verified",
  [PermissionLevel.Verified]: "Verified",
  [PermissionLevel.Helper]: "Helper",
  [PermissionLevel.Admin]: "Admin",
};

export async function myAccount(): Promise<MyAccount> {
  let response = await apiFetch("meuser");
  if (response.ok) {
    return await response.json();
  } else {
    throw new Error(await response.text());
  }
}

export async function amISignedIn(): Promise<boolean> {
  const res = await apiFetch("amiin");
  if (!res.ok) {
    return false;
  }
  const text = await res.text();
  if (text == "yep") {
    return true;
  } else {
    return false;
  }
}

export async function logOut(): Promise<Response> {
  return await apiFetch("logout", {
    method: "POST",
  });
}
