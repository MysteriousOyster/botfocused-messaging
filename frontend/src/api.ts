/*
 * Copyright (C) 2026 Leif Barton
 * Licensed under the Open Software License 3.0
 */
 
import { API_URL } from "./config";

export async function isApiHealthy() {
  try {
    const response = await fetch(API_URL);
    return response.ok && await response.text() === "hello!";
  } catch (e) {
    console.error(e);
    return false;
  }
}