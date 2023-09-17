
pub struct person {
    pub name: String,
    pub age :u32
 }

impl person{
    pub fn name_age_of_person(self) {
        print!("{} age is {}",self.name,self.age);
     }
 }
