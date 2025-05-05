import { invoke } from "@tauri-apps/api/core";
import { Product } from "./lib";

export async function listProducts(): Promise<Product[]> {
  return await invoke("list_products")
}

export async function createProduct(product: Product): Promise<void> {
  return await invoke("create_product", { product })
}

export async function updateProduct(product: Product): Promise<void> {
  return await invoke("update_product", { product })
}

export async function deleteProduct(product: Product): Promise<void> {
  return await invoke("delete_product", { product })
}
