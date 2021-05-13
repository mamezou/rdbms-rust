// fn main() {
//     println!("Hello, world!");
// }
pub struct PageId(pub u64);
pub struct DiskManager {
    // ヒープファイルのファイルディスクリプタ
    heap_file:File,
    // 裁判するページIDを決めるカウンタ
    next_page_id:u64,
}
impl DiskManager {
    // コンストラクタ
    pub fn new(data_file:File) -> io::Result<Self>{
        // ファイルサイズを取得
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id
            = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }
    // ファイルパスを指定して開く
    pub fn open(data_file_path: impl AsRef<Path>)
        -> io::Result<Self>{
            let heap_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(heap_file_path)?;
            self::new(heap_file)
    }
    // 新しいページIDを採番する
    pub fn allocate_page(&mut self)-> PageId{
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }
    // ページのデータを読み出す
    pub fn read_page_data(&mut self,
        page_id:PageId,
        data: &[u8]) -> io::Result<()>{
            // ...
    }
    pub fn write_page_data(&mut self,
        page_id: PageId, data: &[u8]) -> io::Result<()>{
            // オフセットを計算
            let offset = PAGE_SIZE as u64 * page_id.to_u64();
            // ページ先頭へシーク
            self.heap_file.seek(SeekFrom::Stract(offset))?;
            // データを読み出す
            self.heap_file.read_exact(data)
        }
    // ...
}