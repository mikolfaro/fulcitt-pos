import { invoke } from "@tauri-apps/api/core";
import { Product } from "./lib";

export async function listProducts(): Promise<Product[]> {
  return await invoke("list_products")
}

export async function createProduct(product: Product): Promise<void> {
  return await invoke("create_product", { product })
}
