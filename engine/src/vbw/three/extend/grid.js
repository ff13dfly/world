import * as THREE from "three";

const self = {
    /*空间格栅的实现,用于精确显示格栅供对齐
     *@param	cen		array		//[x,y,z]类型的坐标点
     * @param	ax			string		//['x','y','z']表示和ax的轴垂直
     * @param	dv		number	//从ax正方向向反方向看去，位于右侧的轴为"转换x轴"，和该轴垂直的线间距
     * @param	dp		number	//从ax正方向向反方向看去，位于右侧的轴为"转换x轴"，和该轴平行的线间距
     * @param	cv			number	//dv间距的轴数量
     * 	@param	cp			number	//dp间距的轴数量
     * 	@param	color	string		//格栅绘制的颜色
     *  @param	cornor		boolean	//基点坐标定位是否从左下角开始
     * 
     * return 
     * three.group		//three.js的group对象
     * */
    get: (cen, ax, dv, dp, cv, cp, color, cornor, dash) => {
        //计算获取的格栅的左下角坐标点
        let start = [0, 0, 0]
        if (!cornor) {
            switch (ax) {
                case 'x':
                    start[0] = cen[0];																	//指定轴
                    start[1] = cen[1] - ((cv % 2) ? (cv - 1) * 0.5 * dv : (cv * 0.5 - 0.5) * dv);		//转换X轴
                    start[2] = cen[2] - ((cp % 2) ? (cp - 1) * 0.5 * dp : (cp * 0.5 - 0.5) * dp);		//转换Y轴
                    break;
                case 'y':
                    start[0] = cen[0] - ((cp % 2) ? (cp - 1) * 0.5 * dp : (cp * 0.5 - 0.5) * dp);		//转换Y轴
                    start[1] = cen[1];																	//指定轴												
                    start[2] = cen[2] - ((cv % 2) ? (cv - 1) * 0.5 * dv : (cv * 0.5 - 0.5) * dv);		//转换X轴
                    break;
                case 'z':
                    start[0] = cen[0] - ((cv % 2) ? (cv - 1) * 0.5 * dv : (cv * 0.5 - 0.5) * dv);		//转换X轴
                    start[1] = cen[1] - ((cp % 2) ? (cp - 1) * 0.5 * dp : (cp * 0.5 - 0.5) * dp);		//转换Y轴
                    start[2] = cen[2];																	//指定轴			
                    break;
                default:
                    break;
            }
        } else {
            start = cen;
        }
        const lines = new THREE.Group();			//所有的line放到一个group里,方便操作

        //区分是否为虚线
        let mm = null;
        if (dash) {
            mm = self.getLineDashedMaterial(color, 0.2 * dv, 0.8 * dv, 1);
        } else {
            mm = self.getLineBasicMaterial(color, 1);
        }

        const mz = (cp - 1) * dp, my = (cv - 1) * dv;
        let pa = [0, 0, 0], pb = [0, 0, 0];
        for (let i = 0; i < cv; i++) {
            switch (ax) {
                case 'x':
                    pa = [start[0], start[1] + i * dv, start[2]];
                    pb = [start[0], start[1] + i * dv, start[2] + mz];
                    break;
                case 'y':
                    pa = [start[0], start[1], start[2] + i * dv];
                    pb = [start[0] + mz, start[1], start[2] + i * dv];
                    break;
                case 'z':
                    pa = [start[0] + i * dv, start[1], start[2]];
                    pb = [start[0] + i * dv, start[1] + mz, start[2]];
                    break;
                default:
                    break;
            }

            lines.add(self.line(pa, pb, mm));
        }
        for (let i = 0; i < cp; i++) {
            switch (ax) {
                case 'x':
                    pa = [start[0], start[1], start[2] + i * dp];
                    pb = [start[0], start[1] + my, start[2] + i * dp];
                    break;
                case 'y':
                    pa = [start[0] + i * dp, start[1], start[2]];
                    pb = [start[0] + i * dp, start[1], start[2] + my];
                    break;
                case 'z':
                    pa = [start[0], start[1] + i * dp, start[2]];
                    pb = [start[0] + my, start[1] + i * dp, start[2]];
                    break;
                default:
                    break;
            }
            lines.add(self.line(pa, pb, mm));
        }

        return lines;
    },
    valid: (params) => {

        return true;
    },

    //提供standard的数据输出，可以进行比较处理，也供valid来使用
    sample: () => {
        return {
            size: [],
        }
    },
}

const extend_grid = {
    create: (params) => {
        if (!self.valid(params)) return { error: "Invalid parameters to create BOX." };

        //const { size } = params;
        //return new THREE.BoxGeometry(size[0], size[1], size[2]);
        return null;
    },
    standard: () => {
        return self.sample();
    },
};

export default extend_grid;