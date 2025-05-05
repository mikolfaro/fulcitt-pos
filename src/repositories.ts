import { invoke } from "@tauri-apps/api/core";
import { Product } from "./lib";

export async function listProducts(): Promise<Product[]> {
  return await invoke("list_products")
  //
  // return Promise.resolve([{
  //   id: 1,
  //   name: "Polenta",
  //   price: 4.50,
  //   category: "Cibo",
  // }])
}
