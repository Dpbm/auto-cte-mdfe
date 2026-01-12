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
  deliveries: Delivery[];
  license_plate: string;
  total_price: number;
  total_cubicage: number;
};

export type Delivery = {
	danfe: string[];
	key: string[];
	to: string;
	quantity: number;
	price: number;
	cubicage: number;
};


export type LoadData = {
	[key:string]:DataByCarrier;
};

export type DataByCarrier = {
  loads: LoadsByNumber,
  sequence: number[],
};

export type LoadsByNumber = {
	[key: number]: Load;
};

export type RateioData = {
  loads: LoadData;
  errors: string[];
};
