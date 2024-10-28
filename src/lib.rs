use std::{rc::Rc, sync::Arc};

#[derive(Clone)]
pub struct RcId {
    rc: Rc<()>,
}

impl PartialEq for RcId {
    fn eq(&self, other: &RcId) -> bool {
        Rc::ptr_eq(&self.rc, &other.rc)
    }
}

impl Eq for RcId { }

impl RcId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            rc: Rc::new(()),
        }
    }
}

#[derive(Clone)]
pub struct RcUniq<T> {
    id: RcId,
    inner: T,
}

impl<T> PartialEq for RcUniq<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for RcUniq<T> { }

impl<T> RcUniq<T> {
    pub fn new(inner: T) -> Self {
        Self {
            id: RcId::new(),
            inner,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn id(&self) -> &RcId {
        &self.id
    }
}

impl<T> AsRef<T> for RcUniq<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T: Copy> RcUniq<T> {
    pub fn get(&self) -> T {
        self.inner
    }
}

#[derive(Clone)]
pub struct ArcId {
    arc: Arc<()>,
}

impl PartialEq for ArcId {
    fn eq(&self, other: &ArcId) -> bool {
        Arc::ptr_eq(&self.arc, &other.arc)
    }
}

impl Eq for ArcId { }

impl ArcId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            arc: Arc::new(()),
        }
    }
}