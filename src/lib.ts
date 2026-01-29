export interface Product {
  id: number;
  name: string;
  price: number;
  category: string;
  has_deposit: boolean;
}
export type UnsavedProduct = Omit<Product, "id">;

export interface CartItem {
  id: number;
  name: string;
  price: number;
  quantity: number;
  hasDeposit?: boolean;
}

export interface AppMessage {
  type: string;
  message: string;
}

export enum PaymentMethod {
  Card = "Card",
  Cash = "Cash",
}
