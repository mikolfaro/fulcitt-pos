export interface Product {
  id: number,
  name: string,
  price: number,
  category: string
}
export type UnsavedProduct = Omit<Product, 'id'>

export interface CartItem {
  id: number,
  name: string,
  price: number,
  quantity: number
}
