export class SelectedImage {
  constructor(public file: File, public id: string) {}
}

export class ImageResult {
  constructor(public success: SuccessResp[], public fail: FailResp[]) {}
}

export class SuccessResp {
  public hide = false;
  constructor(
    public id: string,
    public trackNumber: string,
    public file: File,
    public rect: Rectangle
  ) {}
}

export class Rectangle {
  public top: number = 0;
  public right: number = 0;
  public bottom: number = 0;
  public left: number = 0;
}

export class FailResp {
  public hide = false;
  constructor(public id: string, public file: File) {}
}

export class BinaryImage {
  constructor(public id: string, public bytes: Uint8Array) {}
}
