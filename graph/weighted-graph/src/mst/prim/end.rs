use std::cmp::Ordering;

// 与权重大小序相反的末端点
#[derive(Eq)]
pub(super) struct End {
    weight: i32,
    id: u32,
}

impl PartialEq for End {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for End {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for End {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

impl End {
    pub(super) const fn new(weight: i32, id: u32) -> Self {
        Self { weight, id }
    }

    // 生成0权重的启动点
    pub(super) const fn forge(id: u32) -> Self {
        Self { weight: 0, id }
    }

    pub(super) const fn id(&self) -> u32 {
        self.id
    }
}
