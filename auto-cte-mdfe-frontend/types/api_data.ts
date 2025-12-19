export type Data = {
  danfe: string;
  to: string;
  by: string;
  load_number: number;
  cubicage: number;
  quantity: number;
  price: number;
};

export type Load = {
  data: Data[];
  license_plate: string;
  total_price: number;
  total_cubicage: number;
};

type EmailLoadData = {
  load_number:number;
  price: number;
  license_plate:string;
};

export type LoadData = {
    [key:number]:Load;
};

type EmailData = {
    [key:number]:EmailLoadData;
};

export type RateioData = {
  loads: LoadData;
  email_data: EmailData;
  errors: string[];
};